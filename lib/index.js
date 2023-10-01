const fs = require("fs");

const express = require("express");
const app = express();
const http = require("http");
const { Server } = require("socket.io");
const cors = require("cors")



var addon = require('../native');

console.log(addon.hello());




app.use(cors())

const server = http.createServer(app)

const io = new Server(server, {
    cors: {
        origin: "http://192.168.15.179:3000",
        methods: ["GET", "POST"],
    },
});

io.on("connection", (socket) => {
    console.log(`User Connected: ${socket.id}`)

    socket.on("send_message", (data) => {
        console.log(data);
    })

    socket.on("upload", (file, name, callback) => {
        console.log(file);
        fs.writeFile("./midi_files/" + name, file, (err) => {
            callback({ message: err ? "failure" : "success" });
        });
    })

    socket.on("get_files", (callback) => {
        fs.readdir("./midi_files", (err, files) => {
            if (err) {
                return console.log('Unable to scan directory: ' + err);
            } 
            callback(files);
        })
        
    })

    socket.on("get_tuning_values", (octave, callback) => {
        fs.readFile("./tuning.txt", "utf8", (err, data) => {
            if (err) {
                return cpmsple.log('Unable to read tuning')
            }
            const allTuning = data.split("\n")
            const upper = allTuning.map((e)=> {return e.split(",")[0]})
            const lower = allTuning.map((e)=> {return e.split(",")[1]})
            callback(upper.slice((octave-1)*12,octave*12),lower.slice((octave-1)*12,octave*12))
        })
    })
});

server.listen(3001, () => {
    console.log("server is running")
})