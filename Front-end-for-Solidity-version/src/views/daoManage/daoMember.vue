<template>
  <div class="dao-member">
    <div class="members">
<!--      <div class="select animated zoomIn">-->
<!--        &lt;!&ndash; You can toggle select (disabled) &ndash;&gt;-->
<!--        <input type="radio" name="option">-->
<!--        <i class="toggle icon icon-arrow-down"></i>-->
<!--        <i class="toggle icon icon-arrow-up"></i>-->
<!--        <span class="placeholder">Choose...</span>-->
<!--        <label class="option">-->
<!--          <input type="radio" name="option">-->
<!--          <span class="title animated fadeIn"><i class="icon icon-speedometer"></i>Speedometer</span>-->
<!--        </label>-->
<!--        <label class="option">-->
<!--          <input type="radio" name="option">-->
<!--          <span class="title animated fadeIn"><i class="icon icon-fire"></i>Fire</span>-->
<!--        </label>-->
<!--        <label class="option">-->
<!--          <input type="radio" name="option" >-->
<!--          <span class="title animated fadeIn"><i class="icon icon-handbag"></i>Handbag</span>-->
<!--        </label>-->
<!--        <label class="option">-->
<!--          <input type="radio" name="option">-->
<!--          <span class="title animated fadeIn"><i class="icon icon-badge"></i>Badge</span>-->
<!--        </label>-->
<!--      </div>-->
      <div class="item" v-for="(item,index) in membersList" :key="index">
        <div class="logo">
          <img src="../../assets/imgs/header-icon.png" alt="">
        </div>
        <div class="member-info">
          <div class="name">Address</div>
          <div class="address">{{item}}</div>
        </div>
      </div>
    </div>
    <div class="nodata" v-show="membersList.length==0">
      No Data
    </div>
  </div>
</template>

<script>
export default {
  name: "daoMember",
  props:["membersList"]
}
</script>

<style lang="scss" scoped>
.dao-member{
  .members{
    display: flex;
    flex-wrap: wrap;
    .item{
      margin: 20px;
      width: 252px;
      height: 80px;
      background: #f5f5f5;
      border-radius: 10px;
      display: flex;
      align-items: center;
      padding: 10px;
      .logo{
        width: 40px;
        height: 40px;
        img{
          width: 100%;
        }
      }
      .member-info{
        .name{
          font-weight: bold;
          line-height: 30px;
        }
        .address{
          font-size: 12px;
          word-break: break-all;
        }
      }
    }
  }
}
.select {
  position: relative;
  overflow: hidden;
  display: block;
  margin: auto;
  width: 330px;
  height: 100%;
  border-bottom: 0px;
  border-radius: 3px;
  font-size: 12px;
  box-shadow: 0px 1em 2em -1.5em rgba(0, 0, 0, .5);
  >i.toggle {
    position: absolute;
    z-index: 4;
    right: 1.5em;
    top: 1.6em;
    color: #ccc
  }
  .title,
  .placeholder {
    position: relative;
    display: block;
    width: 100%;
    height: 100%;
    padding: 1.5em 2em;
    background: rgba(255, 255, 255, 1);
    border-top: 1px solid rgba(0, 0, 0, .05);
    cursor: pointer;
  }
  &>input {
    position: absolute;
    left: 0px;
    top: 0px;
    z-index: 1;
    width: 100%;
    height: 100%;
    display: block;
    opacity: 0;
    cursor: pointer;
    &:checked {
      ~i.toggle.icon-arrow-down {
        display: none;
      }
      ~i.toggle.icon-arrow-up {
        display: block;
      }
      div.options label.option .title {
        display: none!important;
      }
    }
    &:not(:checked) {
      z-index: 4;
      ~label.option {
        >span.title {
          display: none;
        }
      }
      ~i.toggle.icon-arrow-up {
        display: none;
      }
      ~i.toggle.icon-arrow-down {
        display: block;
      }
    }
    &:disabled {
      cursor: no-drop;
    }
  }
  &>span.placeholder {
    position: relative;
    z-index: 0;
    display: inline-block;
    width: 100%;
    color: #999;
    border-top: 0px;
  }
  label.option {
    display: block;
    overflow: hidden;
    z-index: 1;
    width: 100%;
    transition: all 1s ease-out;
    span.title {
      position: relative;
      z-index: 2;
      transition: background .3s ease-out;
      i.icon {
        padding-right: 8px;
        color: rgba(146, 168, 209, 1);
      }
      &:hover {
        color: #999;
        background: rgba(255, 255, 255, .3);
        box-shadow: inset 0px 1px 0px rgba(0, 0, 0, .1);
      }
    }
    input {
      display: none;
      &:checked~span.title {
        position: absolute;
        display: block;
        z-index: 3;
        top: 0px;
        font-size: 12px;
        background: #fff;
        border-top: 0px;
        box-shadow: none;
        color: inherit;
        width: 100%;
      }
      &:disabled {
        &~span.title {
          background: #f9f9f9!important;
          color: #aaa;
          &:hover {
            color: #aaa;
            background: none;
            cursor: no-drop;
          }
        }
      }
    }
  }
} // Pure css select 2 TOGGLE
.daoMember {
  // Start | for demo

  .select2 {
    position: relative;
    overflow: hidden;
    display: block;
    margin: auto;
    width: 330px;
    height: 100%;
    border-bottom: 0px;
    border-radius: 3px;
    font-size: 12px;
    box-shadow: 0px 1em 2em -1.5em rgba(0, 0, 0, .5);
    i.toggle {
      position: absolute;
      z-index: 4;
      right: 1.5em;
      top: 1.6em;
      color: #ccc
    }
    .title,
    .placeholder {
      position: relative;
      display: block;
      width: 100%;
      height: 100%;
      padding: 1.5em 2em;
      background: rgba(255, 255, 255, 1);
      border-top: 1px solid rgba(0, 0, 0, .05);
      cursor: pointer;
    }
    &>label>input {
      position: absolute;
      left: 0px;
      top: 0px;
      z-index: 2;
      width: 100%;
      height: 100%;
      display: block;
      opacity: 0;
      cursor: pointer;
      &:checked {
        z-index: 2;
        ~i.toggle.icon-plus {
          display: none;
        }
        ~i.toggle.icon-minus {
          display: block;
        }
      }
      &:not(:checked) {
        ~i.toggle.icon-minus {
          display: none;
        }
        ~i.toggle.icon-plus {
          display: block;
        }
        ~label.option input:not(:checked)~.title {
          display: none!important;
        }
      }
      &:disabled {
        cursor: no-drop;
      }
    }
    & label>span.placeholder {
      position: relative;
      z-index: 0;
      display: inline-block;
      width: 100%;
      color: #999;
      border-top: 0px;
    }
    label.option {
      display: block;
      overflow: hidden;
      z-index: 1;
      width: 100%;
      transition: all 1s ease-out;
      span.title {
        position: relative;
        z-index: 2;
        transition: background .3s ease-out;
        i.icon {
          padding-right: 8px;
          color: rgba(146, 168, 209, 1);
        }
        &:hover {
          color: #fff;
          background: rgba(146, 168, 209, .5);
          box-shadow: inset 0px 1px 0px rgba(0, 0, 0, .1);
        }
      }
      input {
        display: none;
        &:checked~span.title {
          position: absolute;
          display: block;
          z-index: 1;
          top: 0px;
          font-size: 12px;
          background: #fff;
          border-top: 0px;
          box-shadow: none;
          color: inherit;
          width: 100%;
        }
        &:disabled {
          &~span.title {
            background: #f9f9f9!important;
            color: #aaa;
            &:hover {
              color: #aaa;
              background: none;
              cursor: no-drop;
            }
          }
        }
      }
    }
  }
}
</style>
