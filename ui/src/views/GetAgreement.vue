<script>
import { ethers } from 'ethers'
import contractABI from '@/assets/AgreementContract.json'

export default {
  data() {
    return {
      stationAddress: '0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f',
      entityAddress: '0x14dC79964da2C08b23698B3D3cc7Ca32193d9955',
      error: '',
      amount: '0 ETH',
      status: 'EMPTY',
    }
  },
  watch: {
    stationAddress(address) {
      if (ethers.isAddress(address)) {
        this.error = ''
      } else {
        this.error = 'Failed to parse station address.'
      }
    },
    entityAddress(address) {
      if (ethers.isAddress(address)) {
        this.error = ''
      } else {
        this.error = 'Failed to parse entity address.'
      }
    },
  },
  methods: {
    getAgreement() {
      const contractAddress = import.meta.env.VITE_AGREEMENT_CONTRACT_ADDRESS
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const contract = new ethers.Contract(contractAddress, contractABI.abi, provider)
      contract
        .get(this.stationAddress, this.entityAddress)
        .then((res) => {
          this.amount = `${ethers.formatEther(res[0])} ETH`
          let status = ''
          switch (res[1]) {
            case 0n: {
              status = 'EMPTY'
              break
            }
            case 1n: {
              status = 'CREATED'
              break
            }
            case 2n: {
              status = 'SIGNED'
              break
            }
          }
          this.status = status
        })
        .catch((error) => {
          this.error = error
        })
    },
  },
}
</script>

<template>
  <h1>Get Agreement</h1>
  <div>
    <p>Amount: {{ amount }}</p>
    <p>Status: {{ status }}</p>
  </div>
  <div>
    <label for="stationAddress">Station address</label>
    <input type="text" name="stationAddress" id="stationAddress" v-model="stationAddress" />
  </div>
  <div>
    <label for="entityAddress">Entity address</label>
    <input type="text" name="entityAddress" id="entityAddress" v-model="entityAddress" />
  </div>
  <div>
    <button type="button" @click="getAgreement">Get</button>
  </div>
  <div>
    <p class="error alert" v-if="error !== ''">{{ error }}</p>
  </div>
</template>

<style scoped>
div {
  width: 75%;
  margin: 0 auto 25px auto;
}

button {
  width: 100%;
  margin: auto;
}

.alert {
  margin-top: 25px;
}
</style>
