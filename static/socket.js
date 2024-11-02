import {io, Manager} from "https://cdn.socket.io/4.8.0/socket.io.esm.min.js"


let socket;
export function connectToSocketIO(url) {

    let token = localStorage.getItem("tokens");
    token = JSON.parse(token).access_token;
    const manager = new Manager(url, {
        query: {
            "token": `Bearer ${token}`
        }
    });
    console.log(token);
    const socket = manager.socket("/", {
        auth: {
            token: `Bearer ${token}`
        }
    })

    console.log("Socket.IO connected", url)
}