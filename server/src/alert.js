const alertIfTime = () => {
    const reminders = Array.from(
            document
            .querySelectorAll("li")
        )
        .map(item => ({
            content: item.querySelector("p").textContent,
            timestamp: +item.getAttribute("data-timestamp"),
            led: +item.getAttribute("data-led"),
        }));

    const currentTime = Math.floor(Date.now() / 1000);

    reminders.forEach(reminder => {
        const differenceSeconds = currentTime - reminder.timestamp;

        if (differenceSeconds < 1 * 60) {
            if (reminder.led != 0) fetch("http://localhost:3000/led/toggle?" + new URLSearchParams({ number: reminder.led })).then(console.log);
            fetch("http://localhost:3000/speak?" + new URLSearchParams({ text: reminder.content })).then(console.log);
        }
    });
};

alertIfTime();

setInterval(() => {
    location.reload();
    alertIfTime();
}, 1 * 60 * 1000);

const updateClock = () => {
    const now = new Date();
    const hours = now.getHours().toString().padStart(2, '0');
    const minutes = now.getMinutes().toString().padStart(2, '0');
    const seconds = now.getSeconds().toString().padStart(2, '0');
    document.querySelector(".clock").innerText = `${hours}:${minutes}:${seconds}`;
};

setInterval(updateClock, 1000);
