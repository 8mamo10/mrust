import React from 'react';
import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';

function App() {

  function executeCommands() {
    invoke('simple_command');
    invoke('command_with_message', {
      message: 'some message'
    }).then(message => console.log('command_with_message', message));
    invoke('command_with_object', {
      message: {
        field_str: 'some message',
        field_u32: 12
      }
    }).then(message => console.log('command_with_object', message));
    for (let arg of [1, 2]) {
      invoke('command_with_error', { arg }).then(message => {
        console.log('command_with_error', message)
      }).catch(message => {
        console.log('command_with_error', message)
      })
    }
    invoke('async_command', { arg: 14 }).then(message => {
      console.log('async_command', message)
    })
  }
  function openDialog() {
    open().then(files => console.log(files))
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <br />
        Hello Tauri
        <button onClick={executeCommands}>Click to execute command</button>
        <br />
        <button onClick={openDialog}>Click to open dialog</button>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
