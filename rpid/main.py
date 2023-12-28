import os

from fastapi import FastAPI
import RPi.GPIO as gpio

leds = [
    (),
    (17, True),
    (18, True),
    (19, True),
]

for led in leds:
    gpio.setmode(gpio.BCM)
    gpio.setup(led[0], gpio.OUT)
    gpio.output(LED_PIN, int(led[1]))

app = FastAPI()

@app.get("/led/toggle")
async def toggle(number: int) -> str:
    global leds
    led = leds[number]
    leds[number] = (led[0], not led[1])

    gpio.output(led[0], int(led[1]))

    turn_off = leds.copy().remove(led)
    del turn_off[0] # ()

    for i, led in enumerate(turn_off):
        gpio.output(led[0], True)
        leds[i + 1][1] = True

    return f"TOGGLE OK, NEW STATE: {'ON' if led[1] else 'OFF'}"

@app.get("/speak")
async def speak(text: str) -> str:
    ret = os.system(f'text2speech "{text}"')

    return f"TEXT {'OK' if ret == 0 else 'FAIL'}, TEXT: {text}";

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="localhost", port=3000)
