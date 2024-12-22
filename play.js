const DATA_URL = '/api/touch_led';
const NUMBER_LEDs = 100;
const LED_SPACING = 2;
const UPDATE_FREQUENCY = 20;
const COLOR_LED = '#ff8888';

const container = document.getElementById('circle-container');
const radius = Math.min(container.offsetWidth, container.offsetHeight) / 2 - LED_SPACING;

let leds = [];

function createLEDs() {
    for (let i = 0; i < NUMBER_LEDs; i++) {
        const led = document.createElement('div');
        led.className = 'led';
        const angle = ((i / NUMBER_LEDs) * 2 * Math.PI) - (Math.PI / 2);
        const x = Math.cos(angle) * radius + radius + LED_SPACING;
        const y = Math.sin(angle) * radius + radius + LED_SPACING;
        led.style.transform = `translate(${x}px, ${y}px)`; container.appendChild(led); leds.push({ element: led, x, y });
    }
}

async function highlightLED(index) {
    // console.log("highlight" + index); 
    await fetch(DATA_URL, {
        method: "POST",
        body: `i=${index}`,
        headers: {
            "Content-type": "application/x-www-form-urlencoded"
        }
    });
    leds.forEach((led, i) => {
        led.element.style.backgroundColor = i === index ? COLOR_LED : 'white';
    });
}

function getClosestLED(x, y) {
    const containerRect = container.getBoundingClientRect();
    const centerX = containerRect.width / 2;
    const centerY = containerRect.height / 2;
    const dx = x - centerX;
    const dy = y - centerY;
    const angle = Math.atan2(dy, dx);
    let index = Math.round((angle + Math.PI / 2) / (2 * Math.PI) * NUMBER_LEDs) % NUMBER_LEDs;
    if (index < 0) { index += NUMBER_LEDs; }
    return index;
}

function handleInteraction(event) {
    const { clientX, clientY
    } = event.touches ? event.touches[0] : event; const containerRect = container.getBoundingClientRect(); const
        x = clientX - containerRect.left; const y = clientY - containerRect.top; const index = getClosestLED(x, y);
    highlightLED(index);
}

function handleClick(event) {
    const { clientX, clientY } = event.touches ? event.touches[0]
        : event; const containerRect = container.getBoundingClientRect(); const x = clientX - containerRect.left; const
            y = clientY - containerRect.top; const index = getClosestLED(x, y); highlightLED(index);
}

container.addEventListener('mousemove', handleInteraction);
container.addEventListener('touchmove', handleInteraction);
container.addEventListener('click', handleClick);
container.addEventListener('touchend', handleClick);

createLEDs();