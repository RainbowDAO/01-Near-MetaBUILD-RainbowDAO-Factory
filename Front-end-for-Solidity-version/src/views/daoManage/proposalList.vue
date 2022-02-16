<template>
  <div class="proposal-list-box">
    <div class="proposal-list">

      <div class="list-item" @click="$router.push({name:'proposalDetail',params:{item:item,address:address,vault:vault}})"
           v-for="(item, index) in proposalList" :key="index">
        <div class="index">
          {{ index + 1 }}
        </div>
        <div class="name-box">
          <div class="header-icon">
            <img src="../../assets/imgs/header-icon.png" alt="">
          </div>
          <div class="name">
            {{ item.title }}
          </div>
        </div>
        <div class="item-content">
          <div class="content">
            <div class="item">
              <div class="name">amount</div>
              <div class="value">
                {{ item.amount }}
              </div>
            </div>
            <div class="item">
              <div class="name">description</div>
              <div class="value">
                {{ item.description }}
              </div>
            </div>
          </div>

          <div class="floor">
            <div class="stage">
              {{ item.state }}
            </div>
            <div class="date">
              block:{{ item.endTime - item.startTime  }}~{{ item.endTime }}
            </div>
          </div>
        </div>
      </div>

    </div>
    <div class="nodata" v-show="!proposalList || proposalList.length==0">
      No Data
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";

export default {
  name: "proposalList",
  props:["proposalList","address","vault"],
  data() {
    return {
      activeIndex: 0,
      navList: [
        {
          name: "ALL",
        },
        {
          name: "Active",
        },
        {
          name: "Pending",
        },
        {
          name: "Succeeded",
        },
        {
          name: "Executed",
        },
        {
          name: "Canceled",
        },
        {
          name: "Defeated",
        },
        {
          name: "Expired",
        },
        {
          name: "Queued",
        }
      ]
    }
  },
  computed: {
    ...mapGetters(['account', 'isConnected'])
  },
  created() {
    console.log(this.proposalList)
  }
}
</script>

<style lang="scss" scoped>
.proposal-list {
  padding: 20px;
    .list-item {
      display: flex;
      padding: 20px;
      align-items: center;
      cursor: pointer;
      margin-top: 30px;
      border: 1px solid #eaeaea;

      .name-box {
        display: flex;
        align-items: center;
        padding: 0 20px;

        .name {
          min-width: 100px;
          font-weight: bold;
          font-size: 18px;
          color: #6919BB;
        }
      }

      .item-content {
        display: flex;
        flex-direction: column;
        justify-content: space-between;

        .content {
          .item {
            display: flex;
            justify-content: space-between;
            align-items: center;

            .name {
              line-height: 30px;
              min-width: 100px;
              font-weight: bold;
            }

            .value {
              max-width: 700px;
              overflow: hidden;
              color: #333;
            }
          }
        }

        .floor {
          display: flex;
          align-items: center;
          color: rgba(255, 174, 37, 0.80);

          .stage {
            margin-right: 30px;
            width: 120px;
            height: 24px;
            text-align: center;
            color: #333;
            font-weight: bold;
            line-height: 24px;
            background: rgba(255, 174, 37, 0.20);
            border: 1px solid rgba(255, 174, 37, 0.50);
            border-radius: 5px;
          }
        }
      }
    }
}
</style>
