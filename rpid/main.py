import os

from fastapi import FastAPI
import RPi.GPIO as gpio

LED_PIN = 17
led_state = False

gpio.setmode(gpio.BCM)
gpio.setup(LED_PIN, gpio.OUT)

gpio.output(LED_PIN, int(led_state))

app = FastAPI()

@app.get("/led/toggle")
async def toggle() -> str:
    global led_state
    led_state = not led_state
    gpio.output(LED_PIN, int(led_state))

    return f"TOGGLE OK, NEW STATE: {'ON' if not led_state else 'OFF'}"

@app.get("/speak")
async def speak(text: str) -> str:
    ret = os.system(f'text2speech "{text}"')

    return f"TEXT {'OK' if ret == 0 else 'FAIL'}, TEXT: {text}";

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="localhost", port=3000)
