This is a game-lobby with some games

Games added

- Tic tac toe

I have used [this article](https://blog.logrocket.com/real-time-chat-app-rust-react/) to get a web socket up and running. I also neeeded to install sqlite by running: sudo apt-get install libsqlite3-dev

# Starting:

- cargo run
- cd ui && npm run dev

# Deployment

The backend is deployed at Render through a Docker file.

The frontend is deployed at [Netlify](https://game-lobby-ttc.netlify.app/), and on [Render](https://game-lobby-ui.onrender.com//).

The frontend can be built by: $cd ui && npm run build

Add the following to ui/.env.development:

- NEXT_PUBLIC_WS_URI="ws:localhost:8000/ws"
- NEXT_PUBLIC_API_URL="http://localhost:8000"

Add the following to ui/.env:

- NEXT_PUBLIC_WS_URI="wss:\<URL-to-backend>/ws"
- NEXT_PUBLIC_API_URL="https://\<URL-to-backend>"
