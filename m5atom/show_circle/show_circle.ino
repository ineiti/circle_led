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
#define NUMPIXELS 100
Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);
Adafruit_NeoPixel led(1, 27, NEO_GRB + NEO_KHZ800);

// This needs to define WIFI_AP and WIFI_PW.
// Don't check into github...
#include "wifi.h"

void setup()
{
    M5.begin(true, false, false);

    wifiMulti.addAP(WIFI_AP, WIFI_PW);
    Serial.print("\nConnecting Wifi...\n");

    delay(50);

    pixels.begin();
    led.begin();

    led.setPixelColor(0, pixels.Color(255, 0, 0));
    led.show();
}

#define DELAYVAL 5

void loop()
{
    if ((wifiMulti.run() == WL_CONNECTED))
    {
        led.setPixelColor(0, pixels.Color(0, 255, 0));
        led.show();

        Serial.print("[HTTP] begin...\n");
        http.begin("http://example.com/index.html");
        Serial.print("[HTTP] GET...\n");
        int httpCode = http.GET();
        if (httpCode > 0)
        {
            Serial.printf("[HTTP] GET... code: %d\n", httpCode);

            if (httpCode == HTTP_CODE_OK)
            {
                String payload = http.getString();
                Serial.println(payload);
            }
        }
        else
        {
            Serial.printf("[HTTP] GET... failed, error: %s\n",
                          http.errorToString(httpCode).c_str());
        }

        http.end();

        Serial.println("Will clear");
        pixels.clear();

        for (int i = 0; i < NUMPIXELS; i++)
        {
            pixels.setPixelColor(i, pixels.Color(0, 2, 0));

            pixels.show();

            delay(DELAYVAL);
        }
    }
    else
    {
        Serial.print("connect failed");
    }

    delay(5000);
}
