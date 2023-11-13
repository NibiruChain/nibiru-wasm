import { Chain, CustomChain, NibiruSigningClient } from "@nibiruchain/nibijs"
import { IWallet, WalletEnum, defaultWalletCtx } from "./types"
import { getChainInfo } from "./keplr"
import { toast } from "react-toastify"
import { Context, ReactNode, createContext, useMemo, useState } from "react"

const CHAIN: Chain = new CustomChain({
  prefix: "nibiru", // e.g. `nibiru`
  shortName: "devnet", // e.g. `itn`
  number: 2, // e.g. `1`
})

const connectWalletToExtension = async (
  wallet: IWallet,
  setWallet: (w: IWallet) => void,
) => {
  if (!wallet.walletType) {
    throw new Error("No Wallet type")
  }

  window.wallet = window[wallet.walletType]

  if (!window.wallet) {
    toast.info("Please install a browser extension.", {
      autoClose: 1200,
    })
    return
  }

  // Connect context with wallet extension

  try {
    await window.wallet.experimentalSuggestChain(getChainInfo(CHAIN))

    // This method will ask the user whether to allow access if they haven't visited this website.
    // Also, it will request that the user unlock the wallet if the wallet is locked.
    await window.wallet.enable(CHAIN.chainId)
  } catch {
    toast.error("Failed to suggest the chain")
    return
  }

  try {
    const signer = window.wallet.getOfflineSigner(CHAIN.chainId)
    const { address } = (await signer.getAccounts())[0]
    const key = await window.wallet.getKey(CHAIN.chainId)
    const newWallet = {
      isConnected: true,
      signer: signer,
      signingClient: await NibiruSigningClient.connectWithSigner(
        CHAIN.endptTm,
        signer,
      ),
      address,
      openSelectWallet: false,
      walletType: wallet.walletType,
      key,
    }

    setWallet(newWallet)

    return newWallet
  } catch {
    toast.error("Validation failed")
  }

  return undefined
}

type ConnectWalletBtnProps = {
  text?: string
  className?: string
  children?: ReactNode
}

export const WalletContext: Context<{
  wallet: IWallet
  setWallet: (w: IWallet) => void
  ConnectWalletBtn: ({ text }: ConnectWalletBtnProps) => JSX.Element
}> = createContext({
  wallet: defaultWalletCtx,
  setWallet: (_) => {},
  ConnectWalletBtn: (_) => (
    <div onClick={() => handleConnectWallet(defaultWalletCtx, (_) => {})}>
      "Connect Wallet"
    </div>
  ),
})

/**
 * Connects the wallet to set up a NibiJS SDK (sdk). Intended to be used
 * in event handling.
 *
 * After connecting, 'handleConnectWallet' "pings" the chain to be certain the
 * endpoints are active. If this is successful, the 'sdk' gets added to the context
 * with wallet-connected account as the signer.
 *
 * @async
 * - Values for the Wallet and corresponding 'useState' setter function.
  The setter can be used to update the context from a child component.
 * @returns {Promise<void>}
 */
export const handleConnectWallet = async (
  wallet: IWallet,
  setWallet: (w: IWallet) => void,
): Promise<void> => {
  if (!wallet.walletType) {
    setWallet({ ...wallet, openSelectWallet: true })
    return
  }
  try {
    await handleConnectWallet(wallet, setWallet)
    return
  } catch (e) {
    throw Error(e as string)
  }
}

export const WalletProvider: FC<{ children?: ReactNode }> = (
  props,
): JSX.Element => {
  const [wallet, setWallet] = useState(defaultWalletCtx)

  const ConnectWalletBtn = ({
    text = "Connect Wallet",
    className,
    children,
  }: ConnectWalletBtnProps) =>
    wallet.isConnected ? (
      <>{children}</>
    ) : (
      <div className="" onClick={() => handleConnectWallet(wallet, setWallet)}>
        HELLO, Connect Wallet
      </div>
    )

  const walletMemo = useMemo(
    () => ({ wallet, setWallet, ConnectWalletBtn }),
    [wallet, setWallet, ConnectWalletBtn],
  )
  const isDialogOpen = useMemo(
    () => wallet.openSelectWallet,
    [wallet.openSelectWallet],
  )

  // useEffect(() => {
  //   ;(async () => {
  //     const existingToken = localStorage.getItem(TOKEN)
  //     if (existingToken) {
  //       await handleConnectWallet(wallet, setWallet, existingToken)
  //     }
  //   })()
  // }, [])

  const handleClose = () => {
    setWallet({ ...wallet, openSelectWallet: false })

    if (wallet.walletType) {
      ;(async () => await handleConnectWallet(wallet, setWallet))()
    }
  }

  return (
    <WalletContext.Provider value={walletMemo}>
      {/* <WalletDialog onClose={handleClose} open={isDialogOpen} /> */}
      {props.children}
    </WalletContext.Provider>
  )
}
