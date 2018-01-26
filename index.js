const electron = require("electron");
const addon = require("./native");

const { app, BrowserWindow, ipcMain } = electron;

let mainWindow;

app.on("ready", () => {
  mainWindow = new BrowserWindow({});
  mainWindow.loadURL(`file://${__dirname}/index.html`);
});


console.log(addon.hello());