<script>
import { ethers } from 'ethers'
import { WalletLocalStorageKey, WalletAccountsKey, WalletPartnersKey } from '@/constants/index'
import { ReadWallet, WriteWallet } from '@/utils/index'

export default {
  data() {
    return {
      menu: 'show',
      wallet: null,
      accountName: '',
      accountPrivateKey: '',
      partnerName: '',
      partnerAddress: '',
      error: '',
    }
  },
  computed: {
    accountsAsArr() {
      return Array.from(this.wallet.get(WalletAccountsKey).entries())
    },
    partnersAsArr() {
      return Array.from(this.wallet.get(WalletPartnersKey).entries())
    },
  },
  watch: {
    menu(val) {
      this.clearAlerts()
      if (val === 'show') this.loadWallet()
    },
    accountName() {
      this.clearAlerts()
    },
    accountPrivateKey() {
      this.clearAlerts()
    },
    partnerName() {
      this.clearAlerts()
    },
    partnerAddress() {
      this.clearAlerts()
    },
  },
  created() {
    this.wallet = new Map()
    this.wallet.set(WalletAccountsKey, new Map())
    this.wallet.set(WalletPartnersKey, new Map())
    this.loadWallet()
  },
  methods: {
    clearAlerts() {
      this.error = ''
    },
    createAccount() {
      const wallet = ethers.Wallet.createRandom()
      this.saveAccount(this.accountName, wallet.privateKey, wallet.address)
      this.accountName = ''
    },
    loadWallet() {
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const walletJSON = localStorage.getItem(WalletLocalStorageKey)
      const wallet = ReadWallet(walletJSON)
      this.wallet = wallet
      const promises = Array.from(wallet.get(WalletAccountsKey).keys()).map(async (name) => {
        let account = this.wallet.get(WalletAccountsKey).get(name)
        const balance = await provider.getBalance(account.address)
        account.balance = ethers.formatEther(balance)
        wallet.get(WalletAccountsKey).set(name, account)
        this.wallet = wallet
        return {}
      })
      Promise.all(promises)
        .then(() => {})
        .catch((error) => {
          this.error = error
        })
    },
    clearWallet() {
      localStorage.removeItem(WalletLocalStorageKey)
      this.wallet = new Map()
      this.wallet.set(WalletAccountsKey, new Map())
      this.wallet.set(WalletPartnersKey, new Map())
    },
    async loadAccount() {
      let wallet
      try {
        wallet = new ethers.Wallet(this.accountPrivateKey)
      } catch (error) {
        this.error = error
        return
      }
      this.saveAccount(this.accountName, wallet.privateKey, wallet.address)
      this.accountName = ''
      this.accountPrivateKey = ''
    },
    chooseMenu(menu) {
      this.menu = menu
    },
    downloadBackup() {
      const walletJSON = WriteWallet(this.wallet)
      const blob = new Blob([walletJSON], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = 'rise-backup.json'
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
    },
    triggerLoadBackup() {
      this.$refs.fileInput.click()
    },
    loadBackup() {
      const file = event.target.files[0]
      if (!file) return
      const reader = new FileReader()
      reader.onload = (e) => {
        try {
          localStorage.setItem(WalletLocalStorageKey, e.target.result)
          this.loadWallet()
        } catch (error) {
          this.error = error
        }
      }
      reader.readAsText(file)
    },
    saveAccount(name, privateKey, address) {
      if (name.length === 0) {
        this.error = 'Account name cannot be empty!'
        throw 'account name cannot be empty'
      }
      if (this.wallet.get(WalletAccountsKey).has(name)) {
        this.error = 'This account name is already taken!'
        throw 'this account name is already taken'
      }
      this.wallet.get(WalletAccountsKey).set(name, {
        name: name,
        privateKey: privateKey,
        address: address,
      })
      const walletJSON = WriteWallet(this.wallet)
      localStorage.setItem(WalletLocalStorageKey, walletJSON)
      this.menu = 'show'
    },
    savePartner(name, address) {
      if (name.length === 0) {
        this.error = 'Partner name cannot be empty!'
        throw 'partner name cannot be empty'
      }
      if (this.wallet.get(WalletPartnersKey).has(name)) {
        this.error = 'This partner name is already taken!'
        throw 'this partner name is already taken'
      }
      if (!ethers.isAddress(address)) {
        this.error = 'Failed to parse partner address.'
        throw 'failed to parse partner address'
      }
      this.wallet.get(WalletPartnersKey).set(name, {
        name: name,
        address: address,
      })
      const walletJSON = WriteWallet(this.wallet)
      localStorage.setItem(WalletLocalStorageKey, walletJSON)
      this.menu = 'show'
    },
    loadPartner() {
      this.savePartner(this.partnerName, this.partnerAddress)
      this.partnerName = ''
      this.partnerAddress = ''
    },
  },
}
</script>

<template>
  <h1>Wallet</h1>
  <div>
    <p v-if="error !== ''" class="error alert">
      {{ error }}
    </p>
  </div>

  <div class="container choose-menu h-scroll-container">
    <button class="choose-menu-create-btn" type="button" @click="chooseMenu('show')">Show</button>
    <button class="choose-menu-create-btn" type="button" @click="chooseMenu('create-account')">
      Create account
    </button>
    <button class="choose-menu-create-btn" type="button" @click="chooseMenu('load-account')">
      Load account
    </button>
    <button class="choose-menu-load-btn" type="button" @click="chooseMenu('load-partner')">
      Load partner
    </button>
  </div>

  <section v-if="menu === 'show'">
    <div class="container">
      <div>
        <h2>Accounts</h2>
      </div>
      <div class="accounts-clear-btn">
        <button type="button" @click="clearWallet">Clear</button>
      </div>
    </div>
    <div class="h-scroll-container">
      <table v-if="accountsAsArr.length">
        <thead>
          <tr>
            <th>Name</th>
            <th>Address</th>
            <th>Balance (ETH)</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="[name, account] in accountsAsArr" :key="name">
            <td>{{ account.name }}</td>
            <td>{{ account.address }}</td>
            <td>{{ account.balance }}</td>
          </tr>
        </tbody>
      </table>
      <p v-else>There are no accounts at the moment.</p>
    </div>
    <div class="container">
      <div>
        <h2>Partners</h2>
      </div>
    </div>
    <div class="h-scroll-container">
      <table v-if="partnersAsArr.length">
        <thead>
          <tr>
            <th>Name</th>
            <th>Address</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="[name, partner] in partnersAsArr" :key="name">
            <td>{{ partner.name }}</td>
            <td>{{ partner.address }}</td>
          </tr>
        </tbody>
      </table>
      <p v-else>There are no partners at the moment.</p>
    </div>
    <div class="container backup">
      <button class="backup-download-btn" type="button" @click="downloadBackup">Backup</button>
      <button class="backup-load-btn" type="button" @click="triggerLoadBackup">Restore</button>
      <input
        ref="fileInput"
        type="file"
        accept=".json"
        style="display: none"
        @change="loadBackup"
      />
    </div>
  </section>
  <section v-if="menu === 'create-account'">
    <h2>Create Account</h2>
    <div class="container">
      <div class="item create-account-input">
        <input
          id="accountName"
          v-model="accountName"
          type="text"
          name="accountName"
          placeholder="Account name"
        />
      </div>
      <div class="item create-account-btn">
        <button type="button" @click="createAccount">Create</button>
      </div>
    </div>
  </section>
  <section v-if="menu === 'load-account'">
    <h2>Load Account</h2>
    <div class="container">
      <div class="item load-account-input">
        <input
          id="accountName"
          v-model="accountName"
          type="text"
          name="accountName"
          placeholder="Account name"
        />
        <input
          id="accountPrivateKey"
          v-model="accountPrivateKey"
          type="text"
          name="accountPrivateKey"
          placeholder="Account private key"
        />
      </div>
      <div class="item load-account-btn">
        <button type="button" @click="loadAccount">Load</button>
      </div>
    </div>
  </section>
  <section v-if="menu === 'load-partner'">
    <h2>Load Partner</h2>
    <div class="container">
      <div class="item load-account-input">
        <input
          id="partnerName"
          v-model="partnerName"
          type="text"
          name="partnerName"
          placeholder="Partner name"
        />
        <input
          id="partnerAddress"
          v-model="partnerAddress"
          type="text"
          name="partnerAddress"
          placeholder="Partner address"
        />
      </div>
      <div class="item load-account-btn">
        <button type="button" @click="loadPartner">Load</button>
      </div>
    </div>
  </section>
</template>

<style scoped>
.create-account-input {
  flex: 8;
}

.create-account-btn {
  margin: 0 0 0 25px;
  flex: 4;
}

.create-account-btn > button {
  width: 100%;
}

.alert {
  margin-top: 25px;
}

.accounts-clear-btn > button {
  padding: 5px 25px 5px 25px;
}

.choose-menu > button {
  padding: 5px 25px 5px 25px;
  width: 100%;
}

.choose-menu-create-btn {
  margin: 0 5px 0 0;
}

.choose-menu-load-btn {
  margin: 0 0 0 5px;
}

.load-account-input {
  flex: 10;
}

.load-account-input #accountName {
  margin: 0 2px 0 0;
}

.load-account-input #accountPrivateKey {
  margin: 0 0 0 2px;
}

.load-account-input #partnerName {
  margin: 0 2px 0 0;
}

.load-account-input #partnerAddress {
  margin: 0 0 0 2px;
}

.load-account-btn {
  margin: 0 0 0 4px;
  flex: 1;
}

.load-account-btn > button {
  width: 100%;
}

.backup {
  margin: 25px 0 0 0;
}

.backup > button {
  padding: 5px 25px 5px 25px;
  width: 100%;
}

.backup-download-btn {
  margin: 0 5px 0 0;
}

.backup-load-btn {
  margin: 0 0 0 5px;
}
</style>
