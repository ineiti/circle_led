/**
 * Connecting through WiFi to the display, and showing it.
 */

#include <M5Atom.h>
#include <Arduino.h>
#include <WiFi.h>
#include <WiFiMulti.h>
#include <WiFiUDP.h>
#include <HTTPClient.h>

// This needs to define WIFI_AP and WIFI_PW.
// Don't check into github...
#include "wifi.h"

#define BASE_NAME "circle.gasser.blue"
#define BASE_URL "https://" BASE_NAME
// #define BASE_NAME "192.168.178.70"
// #define BASE_URL "http://" BASE_NAME ":8080"
#define BASE_UDP_PORT 8081

#define REQUEST_FPS 20
#define REQUEST_INTERVAL 1000 / REQUEST_FPS

#define PIN_STRIP 26
#define PIN_LED 27
#define NUMPIXELS 292
#define CIRCLE_SIZE 200

// Doesn't work because of __enable_irq()!
// #include <PololuLedStrip.h>
// Doesn't work because of RMT_MEM_NUM_BLOCKS_1
// #include "Freenove_WS2812_Lib_for_ESP32.h"

#include <Adafruit_NeoPixel.h>

// The first pixel is covered by tape...
Adafruit_NeoPixel pixels(NUMPIXELS + 1, PIN_STRIP, NEO_GRB + NEO_KHZ800);
Adafruit_NeoPixel led(1, PIN_LED, NEO_GRB + NEO_KHZ800);

static uint8_t hex2u8(const char *c) {
  uint8_t high = *c % 16 + 9 * (*c / 97);
  c++;
  uint8_t low = *c % 16 + 9 * (*c / 97);
  return low + (high << 4);
}

static uint32_t str2pix(const char *c) {
  // return pixels.Color(hex2u8(c) >> 4, hex2u8(c + 2) >> 4, hex2u8(c + 4) >> 4);
  return pixels.Color(hex2u8(c), hex2u8(c + 2), hex2u8(c + 4));
  // return pixels.gamma32(pixels.Color(hex2u8(c), hex2u8(c + 2), hex2u8(c + 4)));
}

#define STATE_WIFI 0
void state_wifi();
#define STATE_UDP_READ 1
void state_udp_read();
#define STATE_SSE_CONNECT 2
void state_sse_connect();
#define STATE_SSE_STREAM 3
void state_sse_stream();
#define STATE_POST_CONNECT 4
void state_post_connect();
#define STATE_POST_REQUEST 5
void state_post_request();

#define REQUEST_POST 0
#define REQUEST_SSE 1
#define REQUEST_UDP 2

int request = REQUEST_UDP;
WiFiMulti wifiMulti;
int state = 0;

int request_start() {
  Serial.printf("Request is %d\n", request);
  for (int i = 0; i < 3; i++){
    pixels.setPixelColor(NUMPIXELS - 1 - i, pixels.ColorHSV(0x7fff, (request == i) << 7, 0x80));
  }

  switch (request) {
    case REQUEST_POST:
      return STATE_POST_CONNECT;
    case REQUEST_SSE:
      return STATE_SSE_CONNECT;
    case REQUEST_UDP:
      return STATE_UDP_READ;
  }
}

void setup() {
  M5.begin(true, false, false);

  wifiMulti.addAP(WIFI_AP, WIFI_PW);
  Serial.printf("\nConnecting to %s / %s...\n", WIFI_AP, WIFI_PW);

  delay(50);

  pixels.begin();
  pixels.setBrightness(128);
  pixels.clear();
  for (uint16_t i = 0; i < NUMPIXELS; i++) {
    pixels.setPixelColor(i, pixels.gamma32(pixels.ColorHSV((i % 8) << 13, 0x7f + (i & 0x10) << 3, 0x7f + (i & 0x20) << 3)));
  }
  pixels.setPixelColor(NUMPIXELS - 1, 0);

  led.begin();
  led.setPixelColor(0, pixels.Color(32, 0, 0));
  led.show();
}

void loop() {
  switch (state) {
    case STATE_WIFI:
      state_wifi();
      break;
    case STATE_UDP_READ:
      state_udp_read();
      break;
    case STATE_SSE_CONNECT:
      state_sse_connect();
      break;
    case STATE_SSE_STREAM:
      state_sse_stream();
      break;
    case STATE_POST_CONNECT:
      state_post_connect();
      break;
    case STATE_POST_REQUEST:
      state_post_request();
      break;
  }

  fetch_button();
}

void show_LEDs(const char *hexes) {
  for (int i = 0; i < CIRCLE_SIZE; i++) {
    pixels.setPixelColor(((i + CIRCLE_SIZE / 2) % CIRCLE_SIZE) + 1,
                         str2pix(hexes + i * 6));
    // pixels.gamma32(str2pix(hexes + i * 6)));
  }
  pixels.show();
}

HTTPClient http;

void state_wifi() {
  if ((wifiMulti.run() == WL_CONNECTED)) {
    led.setPixelColor(0, pixels.Color(0, 32, 0));
    led.show();

    state = request_start();
  } else {
    Serial.printf("WiFi connection to %s / %s failed\n", WIFI_AP, WIFI_PW);

    delay(50);
  }
}

WiFiUDP client_udp;

unsigned long last;

void state_udp_read() {
  // WiFiSTAClass local;
  // Serial.printf("Local IP: %s\n", local.localIP().toString());
  // client_udp.begin(8081);

  client_udp.beginPacket(BASE_NAME, BASE_UDP_PORT);
  client_udp.write(0x30);
  client_udp.endPacket();

  int count = 5;
  while (client_udp.parsePacket() == 0) {
    if (count-- == 0) {
      Serial.printf("%06ld (%03d): Didn't get a reply in 50ms\n", millis(), millis() - last);
      return;
    }
    delay(10);
  }
  int bufLen = CIRCLE_SIZE * 6;
  char buf[bufLen + 1];
  int res = client_udp.read(buf, bufLen);
  if (res != bufLen) {
    Serial.printf("%06ld (%03d): Only got %d out of %d bytes\n", millis(), millis() - last);
  } else {
    show_LEDs(buf);
  }
  // buf[10] = 0;
  // Serial.printf("%06ld: Read %d bytes, starting with: %s\n", millis() - last, res, buf);
  last = millis();
}

WiFiClient client;

unsigned long next_read;
unsigned long read_interval;

void state_sse_connect() {
  http.begin(BASE_URL "/get_circle");

  Serial.println("Connecting to get_circle");
  int httpCode = http.GET();
  if (httpCode > 0) {

    if (httpCode == HTTP_CODE_OK) {
      client = http.getStream();
      state = STATE_SSE_STREAM;
      read_interval = REQUEST_INTERVAL;
      next_read = millis() + read_interval;
    } else {
      Serial.printf("HTTPCode is %d\n", httpCode);
    }
  } else {
    Serial.printf("[HTTP] GET... failed, error: %s\n",
                  http.errorToString(httpCode).c_str());
  }
}

void state_sse_stream() {
  unsigned long start = millis();
  if (start < next_read) {
    delay(next_read - start);
  } else {
    Serial.printf("Out of sync by %d - interval: %d\n", next_read - start, read_interval);
  }

  int bufLen = CIRCLE_SIZE * 6 + 15;
  if (client.available() == 0) {
    int loop = REQUEST_INTERVAL;
    for (; client.available() < bufLen; loop--) {
      if (loop == 0) {
        Serial.println("Didn't get any bytes after 500ms - reconnecting");
        state = STATE_SSE_CONNECT;
        return;
      }
      delay(10);
      next_read += 10;
    }
    Serial.printf("%05ld: No bytes available for %d loops\n", millis(), REQUEST_INTERVAL - loop);
    read_interval += 5;
  } else if (client.available() < 2 * bufLen) {
    read_interval += 2;
  } else if (client.available() < 3 * bufLen) {
    read_interval++;
    // } else if (client.available() < 4 * bufLen) {
    //   read_interval++;
  } else if (client.available() >= 4 * bufLen && read_interval > 20) {
    read_interval--;
  }
  next_read += read_interval;

  uint8_t buf[bufLen + 1];
  int read = client.read(buf, bufLen);
  if (read != bufLen) {
    Serial.printf("%05ld: Only read %d instead of %d bytes\n", millis(), read, bufLen);
    return;
  }
  buf[read] = 0;

  char *hexes = (char *)buf + 11;
  // Serial.println((char*)buf);
  // Serial.println(hexes);

  show_LEDs(hexes);

  // buf[20] = 0;
  // Serial.printf("%s\n%s\n", buf, hexes);

  Serial.printf("%05ld + %2d: %05ld, avail: %f\n", millis(), read_interval, next_read, client.available() / (float)bufLen);
  if (client.available() == 0) {
    next_read += 10;
  }
}

void state_post_connect() {
  http.begin(BASE_URL "/api/get_circle");
  http.setReuse(true);

  Serial.println("Connecting to /api/get_circle");
  int httpCode = http.POST("");
  if (httpCode > 0) {

    if (httpCode == HTTP_CODE_OK) {
      client = http.getStream();
      state = STATE_POST_REQUEST;
    }
  } else {
    Serial.printf("[HTTP] POST... failed, error: %s\n",
                  http.errorToString(httpCode).c_str());
  }
}

void state_post_request() {
  unsigned long start = millis();
  int httpCode = http.POST(String(""));
  // Serial.printf("[HTTP] POST... code: %d\n", httpCode);

  if (httpCode > 0) {

    if (httpCode == HTTP_CODE_OK) {
      String payload = http.getString();

      // const char *hexes = payload.c_str() + 1;
      // Serial.println(payload);
      // Serial.println(hexes);
      show_LEDs(payload.c_str() + 1);
    }
  } else {
    Serial.printf("[HTTP] POST... failed, error: %s\n",
                  http.errorToString(httpCode).c_str());

    state = STATE_POST_CONNECT;
    return;
  }

  // http.end();

  unsigned long stop = millis();
  Serial.printf("POST request duration: %ld..%ld = %ld\n", start, stop, stop - start);
  stop = millis();
  if (stop < start + REQUEST_INTERVAL) {
    delay(REQUEST_INTERVAL - (stop - start));
  }
}

void fetch_button() {
  if (M5.Btn.read() == 1) {
    http.begin(BASE_URL "/api/game_reset");
    led.setPixelColor(0, pixels.Color(32, 32, 0));
    led.show();
    int httpCode = http.POST(String(""));
    led.setPixelColor(0, pixels.Color(0, 32, 0));
    led.show();
    // Serial.printf("Sent reset with code: %d\n", httpCode);
    request = (request + 1) % 3;
    state = request_start();

    delay(1000);
  }
}