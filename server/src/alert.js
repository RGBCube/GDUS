const alertIfTime = () => {
    const reminders = Array.from(
            document
            .querySelectorAll("li")
        )
        .map(item => {
            content: item.querySelector("p").textContent,
            timestamp: +item.getAttribute("data-timestamp"),
            led: +item.getAttribute("data-led"),
        });

    const currentTime = Math.floor(Date.now() / 1000);

    reminders.forEach(reminder => {
        const differenceSeconds = currentTime - reminder.timestamp;

        if (differenceSeconds < 1 * 60) {
            if (reminder.led != 0) fetch("http://localhost:3000/led/toggle?" + new URLSearchParams({ number: reminder.led }).then(console.log);
            fetch("http://localhost:3000/speak?" + new URLSearchParams({ text: reminder.content })).then(console.log);
        }
    });
};

alertIfTime();

setInterval(() => {
    location.reload();
    alertIfTime();
}, 1 * 60 * 1000);

