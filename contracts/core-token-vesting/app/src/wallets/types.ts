import { OfflineSigner } from "@cosmjs/proto-signing"
import { NibiruSigningClient } from "@nibiruchain/nibijs"

export enum WalletEnum {
  Keplr = "keplr",
  Leap = "leap",
}

export interface IWallet {
  isConnected: boolean
  address: string
  signingClient?: NibiruSigningClient
  signer?: OfflineSigner
  walletType?: WalletEnum
  openSelectWallet: boolean
}

export const defaultWalletCtx: IWallet = {
  isConnected: false,
  address: "",
  signingClient: undefined,
  walletType: undefined,
  openSelectWallet: false,
}
