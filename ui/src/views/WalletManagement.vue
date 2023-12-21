<script>
import { ethers } from 'ethers'
import { AccountsLocalStorageKey } from '../constants/index'

export default {
  data() {
    return {
      menu: 'show', // show / create / load
      accountName: '',
      accounts: null,
      privateKey: '',
      error: '',
    }
  },
  watch: {
    menu() {
      this.error = ''
    },
    accountName() {
      this.error = ''
    },
    privateKey() {
      this.error = ''
    },
  },
  computed: {
    // We need it to show table.
    accountsAsArr() {
      if (!this.accounts) return Array.from([])
      return Array.from(this.accounts.entries())
    },
  },
  methods: {
    createAccount() {
      const wallet = ethers.Wallet.createRandom()
      this.saveAccount(this.accountName, wallet.privateKey, wallet.address, 0.0)
      this.accountName = ''
    },
    loadAccounts() {
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const accountsJSON = localStorage.getItem(AccountsLocalStorageKey)
      if (!accountsJSON) return
      const accountsObj = JSON.parse(accountsJSON)
      const accounts = new Map(Object.entries(accountsObj))
      this.accounts = accounts
      const promises = Array.from(accounts.keys()).map(async (name) => {
        let account = this.accounts.get(name)
        const balance = await provider.getBalance(account.address)
        account.balance = ethers.formatEther(balance)
        accounts.set(name, account)
        this.accounts = accounts
        return {}
      })
      Promise.all(promises)
        .then(() => {})
        .catch((error) => {
          this.error = error
        })
    },
    clearAccounts() {
      localStorage.removeItem(AccountsLocalStorageKey)
      this.accounts = null
    },
    async loadAccount() {
      let wallet
      try {
        wallet = new ethers.Wallet(this.privateKey)
      } catch (error) {
        this.error = error
        return
      }
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const balance = await provider.getBalance(wallet.address)
      this.saveAccount(this.accountName, wallet.privateKey, wallet.address, balance)
      this.accountName = ''
      this.privateKey = ''
    },
    chooseMenu(menu) {
      this.menu = menu
    },
    downloadBackup() {
      const accountsObj = Object.fromEntries(this.accounts)
      const accountsJSON = JSON.stringify(accountsObj)
      const blob = new Blob([accountsJSON], { type: 'application/json' })
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
          const accountsObj = JSON.parse(e.target.result)
          const accounts = new Map(Object.entries(accountsObj))
          localStorage.setItem(AccountsLocalStorageKey, e.target.result)
          this.accounts = accounts
        } catch (error) {
          this.error = error
        }
      }
      reader.readAsText(file)
    },
    saveAccount(name, privateKey, address, balance) {
      if (name.length === 0) {
        this.error = 'Account name cannot be empty!'
        throw 'account name cannot be empty'
      }
      if (!this.accounts) this.accounts = new Map()
      if (this.accounts.has(name)) {
        this.error = 'This account name is already taken!'
        throw 'this account name is already taken'
      }
      this.accounts.set(name, {
        name: name,
        privateKey: privateKey,
        address: address,
        balance: ethers.formatEther(balance),
      })
      const accountsObj = Object.fromEntries(this.accounts)
      const accountsJSON = JSON.stringify(accountsObj)
      localStorage.setItem(AccountsLocalStorageKey, accountsJSON)
    },
  },
  created() {
    this.loadAccounts()
  },
}
</script>

<template>
  <h1>Wallet</h1>
  <div>
    <p class="error alert" v-if="error !== ''">{{ error }}</p>
  </div>

  <div class="container choose-menu">
    <button class="choose-menu-create-btn" type="button" @click="chooseMenu('show')">
      Show accounts
    </button>
    <button class="choose-menu-create-btn" type="button" @click="chooseMenu('create')">
      Create account
    </button>
    <button class="choose-menu-load-btn" type="button" @click="chooseMenu('load')">
      Load account
    </button>
  </div>

  <section v-if="menu === 'show'">
    <div class="container">
      <div>
        <h2>Accounts</h2>
      </div>
      <div class="accounts-clear-btn">
        <button type="button" @click="clearAccounts">Clear</button>
      </div>
    </div>
    <div>
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
    <div class="container backup">
      <button class="backup-download-btn" type="button" @click="downloadBackup">Download</button>
      <button class="backup-load-btn" type="button" @click="triggerLoadBackup">Upload JSON</button>
      <input
        type="file"
        @change="loadBackup"
        accept=".json"
        ref="fileInput"
        style="display: none"
      />
    </div>
  </section>
  <section v-if="menu === 'create'">
    <h2>Create Account</h2>
    <div class="container">
      <div class="item create-account-input">
        <input
          type="text"
          name="accountName"
          id="accountName"
          placeholder="Account name"
          v-model="accountName"
        />
      </div>
      <div class="item create-account-btn">
        <button type="button" @click="createAccount">Create</button>
      </div>
    </div>
  </section>
  <section v-if="menu === 'load'">
    <h2>Load Account</h2>
    <div class="container">
      <div class="item load-account-input">
        <input
          type="text"
          name="accountName"
          id="accountName"
          placeholder="Account name"
          v-model="accountName"
        />
        <input
          type="text"
          name="privateKey"
          id="privateKey"
          placeholder="Private key"
          v-model="privateKey"
        />
      </div>
      <div class="item load-account-btn">
        <button type="button" @click="loadAccount">Load</button>
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

.load-account-input #privateKey {
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
