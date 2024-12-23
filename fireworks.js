const canvas = document.getElementById('fireworks');
const ctx = canvas.getContext('2d');
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

class Particle {
    constructor(x, y, color) {
        this.x = x;
        this.y = y;
        this.color = color;
        this.radius = Math.random() * 2 + 1;
        this.velocity = {
            x: Math.random() * 6 - 3,
            y: Math.random() * 6 - 3
        };
        this.life = 60;
    }

    draw() {
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
        ctx.fillStyle = this.color;
        ctx.fill();
    }

    update() {
        this.x += this.velocity.x;
        this.y += this.velocity.y;
        this.life--;
        this.velocity.y += 0.05; // Gravity
    }
}

let particles = [];

function createFirework(x, y) {
    const color = `hsl(${Math.random() * 360}, 50%, 50%)`;
    for (let i = 0; i < 100; i++) {
        particles.push(new Particle(x, y, color));
    }
}

function animate() {
    ctx.fillStyle = 'rgba(0, 0, 0, 0.1)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    particles = particles.filter(particle => particle.life > 0);

    particles.forEach(particle => {
        particle.draw();
        particle.update();
    });

    if (Math.random() < 0.05) {
        createFirework(Math.random() * canvas.width, Math.random() * canvas.height / 2);
    }
}

const interval = setInterval(animate, 1000 / 60);
