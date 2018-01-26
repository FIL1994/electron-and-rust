const electron = require("electron");
const { hello, threading_hint, my_func } = require("../native");

const now = require("performance-now");

const { app, BrowserWindow, ipcMain } = electron;

let mainWindow;

app.on("ready", () => {
  mainWindow = new BrowserWindow({
    title: "Electron and Rust"
  });
  mainWindow.loadURL(`file://${__dirname}/index.html`);
  mainWindow.on('closed', () => app.quit());
});

console.log("from lib", hello(), threading_hint(), my_func({name: "one"}, "two"));
