import { useContext, JSX } from "react"
import { WalletContext, handleConnectWallet } from "./wallets/context"
import { ToastOptions, toast } from "react-toastify"

export const truncateMiddle = (
  fullStr: string,
  strLen: number,
  separator: string,
) => {
  if (!fullStr || fullStr.length <= strLen) return fullStr

  separator = separator || "..."

  const sepLen = separator.length,
    charsToShow = strLen - sepLen,
    frontChars = Math.ceil(charsToShow / 2),
    backChars = Math.floor(charsToShow / 2)

  return (
    fullStr.substring(0, frontChars) +
    separator +
    fullStr.substring(fullStr.length - backChars)
  )
}

export const truncateHash = (hash: string, length = 15) =>
  truncateMiddle(hash, length, "...")

export const WalletButton = (): JSX.Element => {
  const { wallet, setWallet } = useContext(WalletContext)

  let walletClass: string
  let displayText: string
  let testId: string
  if (wallet.isConnected) {
    walletClass = "wallet-active"
    displayText = truncateHash(wallet.address)
    testId = "btn-connected-wallet"
  } else {
    walletClass = "wallet-inactive"
    displayText = "Connect Wallet"
    testId = "btn-connect-wallet"
  }

  const handleClick = () => {
    if (!wallet.isConnected) {
      ;(async () => await handleConnectWallet(wallet, setWallet))()
      return
    }

    navigator.clipboard.writeText(wallet.address)
    // TODO feat: display a tooltip on hover that tells you what clicking will do.
    // https://github.com/NibiruChain/web-app-nibiru/issues/205
    const toastOptions: ToastOptions = {
      autoClose: 1500,
      pauseOnHover: false,
      pauseOnFocusLoss: false,
      position: "top-center",
    }
    toast.info("Copied address!", toastOptions)
  }

  return wallet.isConnected ? (
    <div onClick={handleClick} className="text-2xl">
      {" "}
      A connected wallet
    </div>
  ) : (
    <div onClick={handleClick} className="text-2xl">
      {" "}
      A disconnected wallet
    </div>
  )
}
