<template>
  <div class="create-dao">

    <div class="create-box">
      <div class="title">
        CREATE DAO
      </div>
      <div class="stage-list">
        <div class="list-item">
          <div class="stage-header">
            <div class="index">
              1
            </div>
            <div class="stage-title">
              Basic Settings
            </div>
          </div>
          <div class="stage-content" v-show="stage==0">
            <div class="stage-panel">
              <div class="item">
                <div class="name">
                  DAO Name
                </div>
                <div class="input">
                  <input type="text" v-model="daoInfo.name" placeholder="Enter">
                </div>
              </div>
              <!--              <div class="item">-->
              <!--                <div class="name">-->
              <!--                  DAO Symbol-->
              <!--                </div>-->
              <!--                <div class="input">-->
              <!--                  <input type="text" v-model="daoInfo.symbol" placeholder="Enter">-->
              <!--                </div>-->
              <!--              </div>-->
              <div class="item">
                <div class="name">
                  DAO Logo
                </div>
                <div class="input">
                  <input type="text" v-model="daoInfo.logo" placeholder="Enter">
                </div>
              </div>
              <div class="item">
                <div class="name">
                  DAO Introduction
                </div>
                <div class="input">
                  <input type="text" v-model="daoInfo.des" placeholder="Enter">
                </div>
              </div>
              <div class="btn-box">
                <div class="sub-btn" @click="stage+=1">
                  Continue
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="list-item">
          <div class="stage-header">
            <div class="index">
              2
            </div>
            <div class="stage-title">
              Token Settings
            </div>
          </div>
          <div class="stage-content animate__animated  animate__fadeIn" v-show="stage==1">
            <div class="stage-panel">

              <div class="item">
                <div class="name">
                  Token Name
                </div>
                <div class="input">
                  <input type="text" v-model="tokenInfo.tokenName" placeholder="Enter">
                </div>
              </div>
              <div class="item">
                <div class="name">
                  Token Symbol
                </div>
                <div class="input">
                  <input type="text" v-model="tokenInfo.symbol" placeholder="Enter">
                </div>
              </div>
              <!--              <div class="item">-->
              <!--                <div class="name">-->
              <!--                  Token Contract Address-->
              <!--                </div>-->
              <!--                <div class="input">-->
              <!--                  <input type="text" v-model="tokenInfo.address" placeholder="Enter">-->
              <!--                </div>-->
              <!--              </div>-->
              <div class="item">
                <div class="name">
                  Token Decimals
                </div>
                <div class="input">
                  <input type="text" v-model="tokenInfo.decimals" placeholder="Enter">
                </div>
              </div>
              <div class="item">
                <div class="name">
                  Token Total Supply
                </div>
                <div class="input">
                  <input type="number" v-model="tokenInfo.totalSupply" placeholder="Enter">
                </div>
              </div>
              <!--              <div class="item">-->
              <!--                <div class="name">-->
              <!--                  Support-->
              <!--                </div>-->
              <!--                <div class="input">-->
              <!--                  <input type="text" v-model="tokenInfo.support" placeholder="Enter">-->
              <!--                </div>-->
              <!--              </div>-->
              <!--              <div class="item">-->
              <!--                <div class="name">-->
              <!--                  Token Manager-->
              <!--                </div>-->
              <!--                <div class="input">-->
              <!--                  <input type="text" v-model="tokenInfo.namager" placeholder="Enter">-->
              <!--                </div>-->
              <!--              </div>-->
              <div class="btn-box">
                <div class="back-btn" @click="stage>0?stage-=1:''">
                  back
                </div>
                <div class="sub-btn" @click="next()">
                  Continue
                </div>
              </div>
            </div>
          </div>
        </div>
        <!--        <div class="list-item" >-->
        <!--          <div class="stage-header">-->
        <!--            <div class="index">-->
        <!--              3-->
        <!--            </div>-->
        <!--            <div class="stage-title">-->
        <!--              Threshold to join DAO-->
        <!--            </div>-->
        <!--          </div>-->
        <!--          <div class="stage-content  animate__animated  animate__fadeIn" v-show="stage==2">-->
        <!--            <div class="stage-panel">-->
        <!--              <p>-->
        <!--                Your Safe will have one or more owners. We have prefilled the first owner with your connected wallet-->
        <!--                details, but you are free to change this to a different owner.-->
        <!--              </p>-->
        <!--              <p>-->
        <!--                Add additional owners (e.g. wallets of-->
        <!--                your teammates) and specify how many of them have to confirm a transaction before it gets executed. In-->
        <!--                general, the more confirmations required, the more secure your Safe is.Learn about which Safe setup to-->
        <!--                use. The new Safe will ONLY be available on-->
        <!--              </p>-->
        <!--              <div class="dao-member">-->
        <!--                <div class="member-header">-->
        <!--                  <div class="name">-->
        <!--                    Owner Name-->
        <!--                  </div>-->
        <!--                  <div class="address">-->
        <!--                    Owner Address-->
        <!--                  </div>-->
        <!--                </div>-->
        <!--                <div class="member-content">-->

        <!--                  <div class="item">-->
        <!--                    <div class="add-btn" @click="memberLength++">-->
        <!--                      ADD MORE-->
        <!--                    </div>-->
        <!--                  </div>-->
        <!--                </div>-->
        <!--              </div>-->
        <!--              <div class="btn-box">-->
        <!--                <div class="back-btn" @click="stage>0?stage-=1:''">-->
        <!--                  back-->
        <!--                </div>-->
        <!--                <div class="sub-btn" @click="next(2)">-->
        <!--                  Continue-->
        <!--                </div>-->
        <!--              </div>-->
        <!--            </div>-->
        <!--          </div>-->
        <!--        </div>-->
        <div class="list-item ">
          <div class="stage-header">
            <div class="index">
              3
            </div>
            <div class="stage-title">
              Termination and liquidation of DAO
            </div>
          </div>
          <div class="stage-content  animate__animated  animate__fadeIn" v-show="stage==2">
            <div class="stage-panel">


              <p>
                You're about to create a new DAO on and will have to confirm a transaction with your currently
                connected wallet. The creation will cost approximately ＜ 0.1 Eth.
              </p>
              <div class="btn-box">
                <div class="back-btn" @click="stage>0?stage-=1:''">
                  back
                </div>
                <div class="sub-btn" @click="createMul">
                  Create
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script>
export default {
  name: "createDao",
  data() {
    return {
      count: 1,
      stage: 0,
      memberLength: 1,
      daoInfo: {
        name: '', logo: '', des: ''
      },
      tokenInfo: {
        tokenName: "",
        symbol: "",
        decimals: 0,
        totalSupply: 0,
        support: 50
      }
    }
  },
  created() {

  },
  methods: {
    next(index) {
      switch (index) {
        case 1:
          break
        case 2:
          break
      }
      this.stage += 1
    },
    createMul() {
      this.tokenInfo.totalSupply = this.tokenInfo.totalSupply + (10 ** this.tokenInfo.decimals).toString().substr(1,(10 ** this.tokenInfo.decimals).toString().length)
      this.$store.dispatch("daoFactory/creatDaoWithNewToken", {
        ...this.tokenInfo,
        ...this.daoInfo
      }).then(() => {
        this.$router.push({
          name: "daoManage"
        })
      })

    }
  }
}
</script>

<style lang="scss" scoped>

.create-dao {
  background: #ffffff;
  box-shadow: 0px 6px 20px 0px rgba(0, 0, 0, 0.05);

  .create-box {
    background: #ffffff;
    padding: 30px 0;
    width: 800px;
    margin: 30px auto;
    min-height: 500px;

    .title {
      width: 291px;
      height: 30px;
      font-size: 30px;
      font-weight: bold;
      text-align: center;
      color: #333333;
      line-height: 45px;
    }

    .stage-list {
      .list-item {
        &:last-child {
          .stage-content::after {
            visibility: hidden;
          }
        }

        .stage-header {
          display: flex;
          align-items: center;
          padding: 20px 0;

          .index {
            width: 30px;
            height: 30px;
            background: #ff1f84;
            border-radius: 50%;
            color: #fff;
            line-height: 30px;
            text-align: center;
            font-size: 20px;
          }

          .stage-title {
            margin-left: 20px;
            height: 19px;
            font-size: 16px;
            font-family: Roboto, Roboto-Bold;
            font-weight: 700;
            text-align: left;
            color: #333333;
            line-height: 19px;
          }
        }

        .stage-content {
          position: relative;
          padding: 20px 60px;

          &:nth-child(1) {
            text-align: center;
          }

          &::after {
            content: '';
            position: absolute;
            left: 16px;
            top: -10px;
            height: 100%;
            width: 1px;
            background: #FF1F84;
          }

          .stage-panel {
            padding: 20px;
            background: #fff;
            border-radius: 5px;
            box-shadow: 2px 2px 10px #eee;

            p {
              padding: 10px 0;
            }

            .name {
              font-weight: bold;
              line-height: 32px;
            }

            .input-title {
              height: 16px;
              font-size: 14px;
              font-family: Roboto, Roboto-Medium;
              font-weight: 500;
              text-align: justify;
              color: #999999;
              line-height: 16px;
            }

            input {
              width: 480px;
              height: 38px;
              background: #ffffff;
              border: 1px solid #eaeaea;
              border-radius: 10px;
              padding: 0 20px;
            }

            .btn-box {
              display: flex;
              align-items: center;

              .back-btn {
                margin-right: 30px;
                cursor: pointer;
                width: 160px;
                height: 40px;
                text-align: center;
                line-height: 40px;
                color: #FF1F84;
                background: #f0f0f0;
                border-radius: 10px;
              }

              .sub-btn {
                margin: 20px 0;
                width: 160px;
                height: 40px;
                background: linear-gradient(90deg, #12c2e9 0%, #c471ed 64%, #f64f59 100%);
                border: 1px solid #eaeaea;
                border-radius: 10px;
                line-height: 40px;
                color: #fff;
                text-align: center;
                cursor: pointer;
              }
            }

            .dao-member {
              .member-header {
                font-weight: bold;
              }

              .static-item {
                display: flex;
                justify-content: space-between;

                svg {

                }
              }

              .member-header, .member-content .item {
                padding: 10px 0;
                display: flex;

                .name {
                  width: 150px;
                }

                .address {
                  width: 600px;
                  margin-left: 8px;
                }
              }

              .member-content {
                input {
                  width: 100%;
                }

                .add-btn {
                  cursor: pointer;
                  text-align: center;
                  line-height: 38px;
                  width: 150px;
                  height: 38px;
                  background: #f0f0f0;
                  border-radius: 10px;
                }
              }
            }

          }
        }
      }
    }
  }

}
</style>
