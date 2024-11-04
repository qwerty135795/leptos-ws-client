import {io, Manager} from "https://cdn.socket.io/4.8.0/socket.io.esm.min.js"
import {receive_users} from "../dist/leptos-ws-chat-bf5a64a6536f6bb1";
//initSync();
let manager, user_socket;
export function connectToSocketIO(url) {

    let token = localStorage.getItem("tokens");
    token = JSON.parse(token).access_token;
    manager = new Manager(url, {
        query: {
            "access_token": `${token}`
        }
    });
    if (user_socket) {
        return;
    }

    user_socket = manager.socket("/users");
    user_socket.on("dialog", (res) => {
        console.log(res);
        receive_users(res);
    })
    console.log("Socket.IO connected", url)
}
export function search_users(username) {
    console.log(username);
    user_socket.emit("find-user", {username, page_size:10});
}
