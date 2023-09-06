import {useEffect} from 'react'
import './App.css'
import {greet} from './pkg/dai_wasm?init'

function App() {
  const open = () => {
    greet("word")
  }

  return (
    <button onClick={open}>
      ddd
    </button>
  )
}

export default App
