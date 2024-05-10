<script>
import router from '@/router'
import { WalletLocalStorageKey, WalletAccountsKey } from '@/constants/index.js'
import { ReadWallet } from '../utils'

export default {
  data() {
    return {
      wallet: null,
      selectedAccount: '',
      selectedAccountAddress: '',
      landings: [],
    }
  },
  computed: {
    accountsAsArr() {
      return Array.from(this.wallet.get(WalletAccountsKey).entries())
    },
  },
  watch: {
    selectedAccount(name) {
      const account = this.wallet.get(WalletAccountsKey).get(name)
      this.selectedAccountAddress = account.address
      this.load(account.address)
    },
  },
  created() {
    this.loadWallet()
    this.load()
  },
  methods: {
    async load(address) {
      try {
        let url
        if (address) url = `/indexer/landings?address=${address}`
        else url = `/indexer/landings`
        let res = await fetch(url, { method: 'GET' })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        this.landings = data
      } catch (e) {
        console.error(e)
        return
      }
    },
    loadWallet() {
      const walletJSON = localStorage.getItem(WalletLocalStorageKey)
      const wallet = ReadWallet(walletJSON)
      this.wallet = wallet
    },
    goToLandingPage(id) {
      router.push({
        name: 'landing',
        params: {
          id,
        },
      })
    },
  },
}
</script>

<template>
  <div class="container">
    <div class="item">
      <select v-model="selectedAccount">
        <option disabled value="" selected>Select an account</option>
        <option v-for="[key, value] in accountsAsArr" :key="key" :value="key">
          {{ value.name }}
        </option>
      </select>
    </div>
  </div>
  <h1>Landings</h1>
  <div class="h-scroll-container">
    <table v-if="landings.length">
      <thead>
        <tr>
          <th>ID</th>
          <th>Drone</th>
          <th>Station</th>
          <th>Landlord</th>
          <th>Taken Off</th>
          <th>Rejected</th>
          <th>Date</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="{ id, drone, station, landlord, is_taken_off, is_rejected, date } in landings"
          :key="station"
          class="mouse-pointer"
          @click="() => goToLandingPage(id)"
        >
          <td>{{ id }}</td>
          <td>{{ `${drone.slice(2, 6)}..${drone.slice(38, 42)}` }}</td>
          <td>{{ `${station.slice(2, 6)}..${station.slice(38, 42)}` }}</td>
          <td>{{ `${landlord.slice(2, 6)}..${landlord.slice(38, 42)}` }}</td>
          <td>{{ is_taken_off ? 'True' : 'False' }}</td>
          <td>{{ is_rejected ? 'True' : 'False' }}</td>
          <td>{{ new Date(date * 1000).toLocaleString() }}</td>
        </tr>
      </tbody>
    </table>
    <p v-else>There are no landings at the moment.</p>
  </div>
</template>

<style scoped>
button {
  width: 100%;
}

select {
  margin: 25px 0 0 0;
  width: 75%;
}
</style>
