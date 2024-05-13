import { WalletAccountsKey, WalletPartnersKey } from '@/constants/index'

export const WriteWallet = (wallet) => {
  // Delete useless balance field from backup.
  for (let [key, value] of wallet.get(WalletAccountsKey)) {
    if (Object.prototype.hasOwnProperty.call(value, 'balance')) {
      delete value.balance
      wallet.get(WalletAccountsKey).set(key, value)
    }
  }
  const walletObj = Object.fromEntries(wallet)
  walletObj[WalletAccountsKey] = Object.fromEntries(walletObj[WalletAccountsKey])
  walletObj[WalletPartnersKey] = Object.fromEntries(walletObj[WalletPartnersKey])
  const walletJSON = JSON.stringify(walletObj)
  return walletJSON
}

export const ReadWallet = (walletJSON) => {
  if (!walletJSON) {
    let wallet = new Map()
    wallet.set(WalletAccountsKey, new Map())
    wallet.set(WalletPartnersKey, new Map())
    return wallet
  }
  const walletObj = JSON.parse(walletJSON)
  const wallet = new Map(Object.entries(walletObj))
  if (wallet.has(WalletAccountsKey)) {
    const accounts = new Map(Object.entries(wallet.get(WalletAccountsKey)))
    wallet.set(WalletAccountsKey, accounts)
  } else {
    wallet.set(WalletAccountsKey, new Map())
  }
  if (wallet.has(WalletPartnersKey)) {
    const partners = new Map(Object.entries(wallet.get(WalletPartnersKey)))
    wallet.set(WalletPartnersKey, partners)
  } else {
    wallet.set(WalletPartnersKey, new Map())
  }
  return wallet
}
