from fastapi import FastAPI
from RPi import GPIO as gpio

LED_PIN = 17

gipo.setmode(gpio.BCM)

state = False

def set(state: bool) -> None:
    gpio.output(LED, int(!state))

set(state)

app = FastAPI()

@app.get("/toggle")
async def toggle() -> None:
    set(!state)
