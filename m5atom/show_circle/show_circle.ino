/**
 * Connecting through WiFi to the display, and showing it.
 */

#include <M5Atom.h>
#include <Arduino.h>
#include <WiFi.h>
#include <WiFiMulti.h>
#include <HTTPClient.h>

// This needs to define WIFI_AP and WIFI_PW.
// Don't check into github...
#include "wifi.h"

#define BASE_URL "https://circle.gasser.blue/"

#define PIN_STRIP 26
#define PIN_LED 27
#define NUMPIXELS 200

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
}

int state = 0;
#define STATE_WIFI 0
void state_wifi();
#define STATE_HTTP_BEGIN 1
void state_http_begin();
#define STATE_CONNECT 2
void state_connect();
#define STATE_STREAM 3
void state_stream();
#define STATE_REQUEST 4
void state_request();
#define STATE_BUTTON 5
void state_button();
WiFiMulti wifiMulti;

void setup() {
  M5.begin(true, false, false);

  wifiMulti.addAP(WIFI_AP, WIFI_PW);
  Serial.printf("\nConnecting to %s / %s...\n", WIFI_AP, WIFI_PW);

  delay(50);

  pixels.begin();
  pixels.setBrightness(128);
  led.begin();

  led.setPixelColor(0, pixels.Color(255, 0, 0));
  led.show();
}

void loop() {
  switch (state) {
    case STATE_WIFI:
      state_wifi();
      break;
    case STATE_REQUEST:
      state_request();
      break;
    case STATE_BUTTON:
      state_button();
      break;
    case STATE_HTTP_BEGIN:
      state_http_begin();
      break;
    case STATE_CONNECT:
      state_connect();
      break;
    case STATE_STREAM:
      state_stream();
      break;
  }
}

HTTPClient http;

void state_wifi() {
  if ((wifiMulti.run() == WL_CONNECTED)) {
    led.setPixelColor(0, pixels.Color(0, 255, 0));
    led.show();

    state++;
  } else {
    Serial.printf("WiFi connection to %s / %s failed", WIFI_AP, WIFI_PW);

    delay(50);
  }
}

void state_http_begin() {
  // http.begin(BASE_URL "api/get_circle");
  http.begin("http://192.168.178.70:8080/get_circle");
  state = STATE_CONNECT;
}

WiFiClient client;

unsigned long connection;

void state_connect() {
  Serial.println("Connecting to get_circle");
  int httpCode = http.GET();
  if (httpCode > 0) {

    if (httpCode == HTTP_CODE_OK) {
      client = http.getStream();
      state = STATE_STREAM;
      connection = millis();
    }
  } else {
    Serial.printf("[HTTP] GET... failed, error: %s\n",
                  http.errorToString(httpCode).c_str());
  }
}

void state_stream() {
  unsigned long start = millis();

  int bufLen = NUMPIXELS * 6 + 15;
  for (int loop = 0; client.available() < bufLen; loop++){
    if (loop > 100){
      state = STATE_CONNECT;
      return;
    }
    delay(10);
  }
  uint8_t buf[bufLen];
  int read = client.read(buf, bufLen);
  buf[read] = 0;

  unsigned long stop = millis();

  const char *hexes = (char*)buf + 9;
  // Serial.println((char*)buf);
  // Serial.println(hexes);
  for (int i = 0; i < NUMPIXELS; i++) {
    pixels.setPixelColor(((i + NUMPIXELS / 2) % NUMPIXELS) + 1,
                          str2pix(hexes + i * 6));
    // pixels.gamma32(str2pix(hexes + i * 6)));
  }
  pixels.show();

  unsigned long stop_all = millis();
  Serial.printf("Got %d bytes in: %02ld / %02ld: %05ld - %05ld..%05ld..%05ld\n", read, stop_all - start, stop - start, connection, start, stop, stop_all);
  stop = millis();
  if (stop < start + 50) {
    delay(50 - (stop - start));
  }
  connection += 50;
}

void state_request() {
  unsigned long start = millis();
  int httpCode = http.POST(String(""));
  // Serial.printf("[HTTP] POST... code: %d\n", httpCode);

  if (httpCode > 0) {

    if (httpCode == HTTP_CODE_OK) {
      String payload = http.getString();

      const char *hexes = payload.c_str() + 1;
      // Serial.println(payload);
      // Serial.println(hexes);
      for (int i = 0; i < NUMPIXELS; i++) {
        pixels.setPixelColor(((i + NUMPIXELS / 2) % NUMPIXELS) + 1,
                             str2pix(hexes + i * 6));
        // pixels.gamma32(str2pix(hexes + i * 6)));
      }
      pixels.show();
    }
  } else {
    Serial.printf("[HTTP] GET... failed, error: %s\n",
                  http.errorToString(httpCode).c_str());
    http.begin(BASE_URL "api/get_circle");
    http.setReuse(true);
  }

  // http.end();

  unsigned long stop = millis();
  Serial.printf("GET request duration: %ld..%ld = %ld\n", start, stop, stop - start);
  stop = millis();
  if (stop < start + 100) {
    delay(100 - (stop - start));
  }

  state = STATE_BUTTON;
}

void state_button() {
  if (M5.Btn.read() == 1) {
    http.begin(BASE_URL "api/game_reset");
    led.setPixelColor(0, pixels.Color(255, 255, 0));
    led.show();
    int httpCode = http.POST(String(""));
    led.setPixelColor(0, pixels.Color(0, 255, 0));
    led.show();
    // Serial.printf("Sent reset with code: %d\n", httpCode);
  }

  state = STATE_REQUEST;
}