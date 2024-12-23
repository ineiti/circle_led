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

WiFiMulti wifiMulti;
HTTPClient http;

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

void setup()
{
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

static uint8_t hex2u8(const char *c)
{
    uint8_t high = *c % 16 + 9 * (*c / 97);
    c++;
    uint8_t low = *c % 16 + 9 * (*c / 97);
    return low + (high << 4);
}

static uint32_t str2pix(const char *c)
{
    // return pixels.Color(hex2u8(c) >> 4, hex2u8(c + 2) >> 4, hex2u8(c + 4) >> 4);
    return pixels.Color(hex2u8(c), hex2u8(c + 2), hex2u8(c + 4));
}

void loop()
{
    if ((wifiMulti.run() == WL_CONNECTED))
    {
        led.setPixelColor(0, pixels.Color(0, 255, 0));
        led.show();

        http.begin(BASE_URL "api/get_circle");
        int httpCode = http.POST(String(""));
        // Serial.printf("[HTTP] POST... code: %d\n", httpCode);

        if (httpCode > 0)
        {

            if (httpCode == HTTP_CODE_OK)
            {
                String payload = http.getString();

                const char *hexes = payload.c_str() + 1;
                // Serial.println(payload);
                // Serial.println(hexes);
                for (int i = 0; i < NUMPIXELS; i++)
                {
                    pixels.setPixelColor(((i + NUMPIXELS / 2) % NUMPIXELS) + 1,
                                         str2pix(hexes + i * 6));
                    // pixels.gamma32(str2pix(hexes + i * 6)));
                }
                pixels.show();
            }
        }
        else
        {
            Serial.printf("[HTTP] GET... failed, error: %s\n",
                          http.errorToString(httpCode).c_str());
        }

        if (M5.Btn.read() == 1)
        {
            http.begin(BASE_URL "api/game_reset");
            led.setPixelColor(0, pixels.Color(255, 255, 0));
            led.show();
            int httpCode = http.POST(String(""));
            led.setPixelColor(0, pixels.Color(0, 255, 0));
            led.show();
            // Serial.printf("Sent reset with code: %d\n", httpCode);
        }

        http.end();
    }
    else
    {
        Serial.printf("WiFi connection to %s / %s failed", WIFI_AP, WIFI_PW);
    }

    delay(50);
}
