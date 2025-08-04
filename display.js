const DATA_URL = '/api/get_circle';
const UPDATE_FREQUENCY = 50;
// This is in pixels
const LED_SIZE = 10;

const container = document.getElementById('circle-container');
const center = Math.min(container.offsetWidth, container.offsetHeight) / 2;
const radius = center - LED_SIZE;

let leds = [];

function createLEDs() {
    for (let i = 0; i < LED_COUNT; i++) {
        const led = document.createElement('div');
        led.width = `${LED_SIZE}px`;
        led.height = `${LED_SIZE}px`;
        led.className = 'led';

        const angle = ((i / LED_COUNT) * 2 * Math.PI) - (Math.PI / 2);
        const x = Math.cos(angle) * radius + center - LED_SIZE / 2;
        const y = Math.sin(angle) * radius + center - LED_SIZE / 2;
        led.style.transform = `translate(${x}px, ${y}px)`;
        leds.push({ element: led, x, y });
        container.appendChild(led);
    }
}

let index = 0;
async function updateLEDs() {
    try {
        const response = await fetch(DATA_URL, {
            method: "POST",
            body: "",
            headers: {
                "Content-type": "application/x-www-form-urlencoded"
            }
        });
        const data = await response.text();
        const leds = document.querySelectorAll('.led');
        for (let i = 0; i < LED_COUNT; i++) {
            leds[i].style.backgroundColor = `#${data.slice(i * 6+1, i * 6 + 7)}`;
        }
    } catch (error) {
        console.error('Error fetching LED data:', error);
    }
}

createLEDs();
setInterval(updateLEDs, 1000 / UPDATE_FREQUENCY);
