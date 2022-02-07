

# lamp-bot
A Telegram bot used to toggle a decorative lamp in my room using a Raspberry Pi 3B+.

I had a nice planet shaped lamp in my room, but I had to plug it in a USB port every time I wanted to turn it on, as it doesn't have a switch. So I made a Telegram bot that runs on a Raspberry Pi 3B+ which turns the lamp on and off.

The lamp is plugged in to a USB port on the Pi, and the bot toggles the lamp by turning the power on and off on the USB ports.

I used [Teloxide](https://github.com/teloxide/teloxide) to create the bot and [uhubctl](https://github.com/mvp/uhubctl) to control the power to the USB ports. I used Rust to create the bot as I want to learn the language.
