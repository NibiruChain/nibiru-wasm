import { useState } from "react"
import reactLogo from "./assets/react.svg"
import viteLogo from "/vite.svg"
// import { WalletSection } from "./wallets/connectWallet"
// import { wallets as keplrWallets } from "@cosmos-kit/keplr"
// import { wallets as cosmostationWallets } from "@cosmos-kit/cosmostation"
// import { wallets as leapWallets } from "@cosmos-kit/leap"
import "./App.css"
// import { ChainProvider } from "@cosmos-kit/react"
import { SignerOptions } from "@cosmos-kit/core"
// import { chains, assets } from "chain-registry"
import { WalletButton } from "./Wallet"
import { WalletProvider } from "./wallets/context"

function App() {
  const [count, setCount] = useState(0)
  const signerOptions: SignerOptions = {}

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Token Vesting POC</h1>

      <WalletProvider>
        <WalletButton />
      </WalletProvider>

      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p>View smart contract source code.</p>
      <p className="read-the-docs text-red-400">Click on the Vite</p>
    </>
  )
}

export default App
