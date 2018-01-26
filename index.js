const electron = require("electron");
const { hello, threading_hint } = require("./native");

const now = require("performance-now");

const { app, BrowserWindow, ipcMain } = electron;

let mainWindow;

app.on("ready", () => {
  mainWindow = new BrowserWindow({});
  mainWindow.loadURL(`file://${__dirname}/index.html`);
});
