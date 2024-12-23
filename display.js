const DATA_URL = '/api/get_circle';
const LED_SPACING = 2;
const UPDATE_FREQUENCY = 20;

const container = document.getElementById('circle-container');
const radius = Math.min(container.offsetWidth, container.offsetHeight) / 2 - LED_SPACING;

function createLEDs() {
    for (let i = 0; i < LED_COUNT; i++) {
        const led = document.createElement('div');
        led.className = 'led';
        const angle = ((i / LED_COUNT) * 2 * Math.PI) - (Math.PI / 2); // Adjust angle to start at top
        const x = Math.cos(angle) * radius + radius + LED_SPACING;
        const y = Math.sin(angle) * radius + radius + LED_SPACING;
        led.style.transform = `translate(${x}px, ${y}px)`;
        container.appendChild(led);
    }
}

function generateGradientData(index) {
    let data = '';
    for (let i = 0; i < LED_COUNT; i++) {
        const ratio = Math.abs(((i + index) % LED_COUNT) / LED_COUNT * 2 - 1);
        const r = Math.floor(255 * ratio).toString(16).padStart(2, '0');
        const g = Math.floor(255 * (1 - ratio)).toString(16).padStart(2, '0');
        const b = '00'; // Keep blue constant for a red-green gradient
        data += `${r}${g}${b}`;
    }
    return data;
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
