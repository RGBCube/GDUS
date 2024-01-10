import os

from fastapi import FastAPI
import RPi.GPIO as gpio

leds = [
    [17, True],
    [18, True],
    [19, True],
]

for led in leds:
    gpio.setmode(gpio.BCM)
    gpio.setup(led[0], gpio.OUT)
    gpio.output(led[0], int(led[1]))

app = FastAPI()

@app.get("/led/toggle")
async def toggle(number: int) -> str:
    global leds

    led = leds[number - 1]
    led[1] = not led[1]
    gpio.output(led[0], int(led[1]))

    for i, iled in enumerate(leds):
        if iled[0] == led[0]: continue

        iled[1] = True
        gpio.output(iled[0], iled[1])

    return f"TOGGLE OK, NEW STATE: {'ON' if led[1] else 'OFF'}"

@app.get("/speak")
async def speak(text: str) -> str:
    with open("speak.txt") as f:
        f.write(text)

    ret = os.system(f'gtts-cli --file speak.txt --lang tr --output text.mp3; pw-play text.mp3')

    return f"TEXT {'OK' if ret == 0 else 'FAIL'}, TEXT: {text}";

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="localhost", port=3000)
