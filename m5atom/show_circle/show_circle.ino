/**
 * Connecting through WiFi to the display, and showing it.
 */

#include <M5Atom.h>
#include <Arduino.h>
#include <WiFi.h>
#include <WiFiMulti.h>
#include <HTTPClient.h>

#include <Adafruit_NeoPixel.h>

WiFiMulti wifiMulti;
HTTPClient http;

#define PIN 26
#define NUMPIXELS 201
Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);
Adafruit_NeoPixel led(1, 27, NEO_GRB + NEO_KHZ800);

// This needs to define WIFI_AP and WIFI_PW.
// Don't check into github...
#include "wifi.h"

void setup()
{
    M5.begin(true, false, false);

    wifiMulti.addAP(WIFI_AP, WIFI_PW);
    Serial.println("\nConnecting Wifi...\n");

    delay(50);

    pixels.begin();
    led.begin();

    led.setPixelColor(0, pixels.Color(255, 0, 0));
    led.show();
}

#define DELAYVAL 5

static uint8_t hex2u8(const char *c)
{
    uint8_t high = *c % 16 + 9 * (*c / 97);
    c++;
    uint8_t low = *c % 16 + 9 * (*c / 97);
    return low + (high << 4);
}

static uint32_t str2pix(const char *c)
{
    return pixels.Color(hex2u8(c) >> 4, hex2u8(c + 2) >> 4, hex2u8(c + 4) >> 4);
    // return pixels.Color(hex2u8(c), hex2u8(c + 2), hex2u8(c + 4));
}

void loop()
{
    if ((wifiMulti.run() == WL_CONNECTED))
    {
        led.setPixelColor(0, pixels.Color(0, 255, 0));
        led.show();

        http.begin("http://fricklebox.fritz.box:8080/api/get_circle");
        int httpCode = http.POST(String(""));
        // Serial.printf("[HTTP] POST... code: %d\n", httpCode);

        if (httpCode > 0)
        {

            if (httpCode == HTTP_CODE_OK)
            {
                String payload = http.getString();
                pixels.clear();

                const char *hexes = payload.c_str() + 1;
                // Serial.println(payload);
                // Serial.println(hexes);
                for (int i = 0; i < NUMPIXELS; i++)
                {
                    pixels.setPixelColor(i + 1, str2pix(hexes + i * 6));
                }
                pixels.show();
            }
        }
        else
        {
            Serial.printf("[HTTP] GET... failed, error: %s\n",
                          http.errorToString(httpCode).c_str());
        }

        if (M5.Btn.wasPressed())
        {
            http.begin("http://fricklebox.fritz.box:8080/api/game_reset");
            http.POST(String(""));
        }

        http.end();
    }
    else
    {
        Serial.print("connect failed");
    }

    delay(50);
}
