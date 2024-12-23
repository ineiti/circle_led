const DATA_URL_POS = '/api/player_pos';
const DATA_URL_CLICK = '/api/player_click';
const LED_SPACING = 2;
const UPDATE_FREQUENCY = 20;

const container = document.getElementById('circle-container');
const radius = Math.min(container.offsetWidth, container.offsetHeight) / 2 - LED_SPACING;

let leds = [];
let PLAYER_LED = 'Red';
let player;

function ledDiv(x, y, name) {
    const led = document.createElement('div');
    led.className = name;
    led.style.transform = `translate(${x}px, ${y}px)`;
    container.append(led);
    return led;
}

function createLEDs() {
    for (let i = 0; i < LED_COUNT; i++) {
        const angle = ((i / LED_COUNT) * 2 * Math.PI) - (Math.PI / 2);
        const x = Math.cos(angle) * radius + radius + LED_SPACING;
        const y = Math.sin(angle) * radius + radius + LED_SPACING;
        const led = ledDiv(x, y, 'led');
        leds.push({ element: led, x, y });
    }
}

async function highlightLED(index) {
    if (index < 0) {
        return;
    }

    await fetch(DATA_URL_POS, {
        method: "POST",
        body: `i=${index}&c=${PLAYER_LED}`,
        headers: {
            "Content-type": "application/x-www-form-urlencoded"
        }
    });
    leds.forEach((led, i) => {
        led.element.style.backgroundColor = i === index ? PLAYER_LED : 'white';
    });
}

function playerLED(color) {
    player = ledDiv(radius * 0.5 + LED_SPACING * 1.5, radius * 0.5 + LED_SPACING * 1.5, 'ledPlayer');
    player.style.backgroundColor = color;
    player.onclick = async () => {
        await fetch(DATA_URL_CLICK, {
            method: "POST",
            body: `c=${PLAYER_LED}`,
            headers: {
                "Content-type": "application/x-www-form-urlencoded"
            }
        });
    };
    player.addEventListener('click', handleClick);
    player.addEventListener('touched', handleClick);
    PLAYER_LED = color;
}

function getClosestLED(x, y) {
    const containerRect = container.getBoundingClientRect();
    const centerX = containerRect.width / 2;
    const centerY = containerRect.height / 2;
    const dx = x - centerX;
    const dy = y - centerY;
    if (Math.hypot(dx, dy) < centerX / 2) {
        return -1;
    }
    const angle = Math.atan2(dy, dx);
    let index = Math.round((angle + Math.PI / 2) / (2 * Math.PI) * LED_COUNT) % LED_COUNT;
    if (index < 0) { index += LED_COUNT; }
    return index;
}

function handleInteraction(event) {
    const { clientX, clientY
    } = event.touches ? event.touches[0] : event;
    const containerRect = container.getBoundingClientRect();
    const x = clientX - containerRect.left;
    const y = clientY - containerRect.top;
    const index = getClosestLED(x, y);
    highlightLED(index);
}

function handleClick(event) {
    const { clientX, clientY } = event.touches ? event.touches[0] : event;
    const containerRect = container.getBoundingClientRect();
    const x = clientX - containerRect.left;
    const y = clientY - containerRect.top;
    const index = getClosestLED(x, y); highlightLED(index);
}

container.addEventListener('mousemove', handleInteraction);
container.addEventListener('touchmove', handleInteraction);

createLEDs();