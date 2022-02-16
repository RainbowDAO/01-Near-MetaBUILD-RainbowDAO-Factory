<template>
  <div class="dao-footer">
    <div class="line"></div>
    <div class="dao-footer-part1">
      <div class="footer-part2-item  animate__animated animate__zoomIn">
        <div class="item-box" @click="navTo('https://www.rainbowcity.io')">
          <div class="title">
            Rainbow City
          </div>
          <div class="link">https://www.rainbowcity.io</div>
        </div>
      </div>
      <div class="footer-part2-item  animate__animated animate__zoomIn animate__delay-0_1s">
        <div class="item-box" @click="navTo('https://www.rainbowdao.io')">
          <div class="title">
            Rainbow DAO
          </div>
          <div class="link">https://www.rainbowdao.io</div>
        </div>
      </div>
      <div class=" footer-part2-item animate__animated animate__zoomIn animate__delay-0_2s">
        <div class="item-box" @click="navTo('https://www.rainbowswap.io')">
          <div class="title">
            Rainbow Swap
          </div>
          <div class="link">https://www.rainbowswap.io</div>
        </div>
      </div>
      <div class=" footer-part2-item animate__animated animate__zoomIn animate__delay-0_3s">
        <div class="item-box" @click="navTo('https://www.rainbowbank.io')">
          <div class="title">
            Rainbow Bank
          </div>
          <div class="link">https://www.rainbowbank.io</div>
        </div>
      </div>
      <div class=" footer-part2-item animate__animated animate__zoomIn animate__delay-0_4s">
        <div class="item-box" @click="navTo('https://www.rainbowusd.io')">
          <div class="title">
            Rainbow Mint
          </div>
          <div class="link">https://www.rainbowusd.io</div>
        </div>
      </div>
      <div class=" footer-part2-item animate__animated animate__zoomIn animate__delay-0_5s">
        <div class="item-box" @click="navTo('https://www.rainbowbridge.io')">
          <div class="title">
            Rainbow Bridge
          </div>
          <div class="link">https://www.rainbowbridge.io</div>
        </div>
      </div>
      <div class=" footer-part2-item animate__animated animate__zoomIn animate__delay-0_6s">
        <div class="item-box" @click="navTo('https://www.rainbowisland.io')">
          <div class="title">
            Rainbow Island
          </div>
          <div class="link">https://www.rainbowisland.io</div>
        </div>
      </div>
      <div class=" footer-part2-item animate__animated animate__zoomIn animate__delay-0_7s">
        <div class="item-box" @click="navTo('https://www.rainbowchain.io')">
          <div class="title">
            Rainbow Chain
          </div>
          <div class="link">https://www.rainbowchain.io</div>
        </div>
      </div>
    </div>
    <div class="dao-footer-part2 ">
      <div class="icon-box">
        <span></span>
        <a class="icon" href="https://t.me/RainbowDAO" target="_blank">
          <svg t="1630481964879" class="icon" viewBox="0 0 1223 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
               p-id="3604" width="48" height="48">
            <path
                d="M1034.968573 961.622321l184.573078-868.139143c7.509214-36.721196 3.128839-63.060335-13.226458-78.90364-16.298409-15.871749-37.830512-18.773036-64.653198-8.76075L57.035613 422.978405c-24.262727 9.215854-40.674912 19.626356-49.549438 31.345282-8.76075 11.718926-9.870066 22.726751-3.128839 33.194141 6.655895 10.46739 20.053016 18.573928 40.162919 24.461835l277.414274 86.38441 643.972026-404.64426c17.635276-11.718926 31.003954-14.136665 40.16292-7.509215 5.887907 4.238155 4.238155 10.46739-5.006143 18.773036L480.140025 674.793085l-20.138348 285.5777c19.256584 0 38.114952-9.102078 56.518216-27.56223l135.59252-130.273493 281.168881 206.759393c53.616929 29.97997 87.52217 14.136665 101.772611-47.672134h-0.085332z"
                p-id="3605" fill="#1296db"></path>
          </svg>
        </a>
      </div>
      <div class="icon-box">
        <span></span>
        <a class="icon" href="https://twitter.com/RainbowcityDAO" target="_blank">
          <svg t="1630482035608" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
               p-id="4479" width="48" height="48">
            <path
                d="M923.2 289.6c0 354.4-232.8 609.6-575.2 620.8-144.8 11.2-244-32.8-332.8-99.2 99.2 11.2 232.8-21.6 298.4-88.8-99.2 0-155.2-56-188-132h88.8C125.6 556.8 58.4 490.4 58.4 380c21.6 11.2 44 21.6 88.8 21.6-77.6-44-121.6-188-66.4-276.8 99.2 110.4 220.8 210.4 421.6 220.8-56-220.8 232.8-332.8 354.4-188 56-11.2 88.8-32.8 132-56-11.2 56-44 88.8-88.8 110.4 44 0 77.6-11.2 110.4-32.8-10.4 44-54.4 88.8-87.2 110.4z"
                fill="#1296db" p-id="4480"></path>
          </svg>
        </a>
      </div>
      <div class="icon-box">
        <span></span>
        <a class="icon" href="https://medium.com/rainbowcity" target="_blank">
          <svg t="1630482306509" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="13894" width="48" height="48"><path d="M121.429333 271.36a39.594667 39.594667 0 0 0-12.928-33.450667l-95.573333-115.2v-17.194666H309.76l229.461333 503.253333 201.728-503.253333H1024v17.194666l-81.792 78.378667a23.936 23.936 0 0 0-9.088 22.954667v576a23.936 23.936 0 0 0 9.088 22.912l79.786667 78.378666v17.194667h-401.493334v-17.194667l82.645334-80.298666c8.106667-8.106667 8.106667-10.496 8.106666-22.954667V332.544l-229.973333 584.021333h-31.018667L182.528 332.544v391.424c-2.218667 16.469333 3.242667 33.024 14.805333 44.928l107.52 130.56v17.152H0v-17.194667l107.52-130.56c11.52-11.861333 16.64-28.586667 13.909333-44.885333V271.36z" fill="#1296db" p-id="13895"></path></svg>
        </a>
      </div>
      <div class="icon-box">
        <span></span>
        <a class="icon" href="https://discord.gg/vbnvFEeYRr" target="_blank">
          <svg t="1630482260015" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="12198" width="48" height="48"><path d="M512.028 171.577a721.806 721.806 0 0 0-346.851 87.467c78.336-69.974 215.267-110.28 215.267-110.28l-12.942-12.942c-128.54 2.276-244.906 91.278-244.906 91.278A1199.73 1199.73 0 0 0 0.142 735.8a331.89 331.89 0 0 0 264.704 127.773l54.045-68.465a289.252 289.252 0 0 1-155.165-104.96 671.915 671.915 0 0 0 696.718 0 289.252 289.252 0 0 1-155.164 104.96l54.044 68.465a331.89 331.89 0 0 0 264.704-127.772 1199.73 1199.73 0 0 0-122.453-508.843S785.21 137.956 656.668 135.68l-12.942 12.942s136.903 40.306 215.24 110.28a721.806 721.806 0 0 0-346.852-87.467M354.588 459.89a92.416 92.416 0 0 1 89.003 96.71 88.946 88.946 0 1 1-177.209 0 92.302 92.302 0 0 1 88.178-96.71m317.184 0a96.996 96.996 0 1 1-88.178 96.71 92.9 92.9 0 0 1 88.206-96.71z" p-id="12199" fill="#1296db"></path></svg>
        </a>
      </div>
      <div class="icon-box">
        <span></span>
        <a class="icon" href="https://github.com/RainbowDAO" target="_blank">
          <svg t="1630482394010" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="16649" width="48" height="48"><path d="M905.55 174.86q26.29-60.57 12.01-115.43Q903.27 4.57 898.69 0q-80 4.57-129.14 34.86-49.14 30.28-50.28 34.85-37.72-16-102.86-20t-72-4q-6.86 0-72 4t-102.86 20q-1.14-4.57-50.29-34.85Q270.12 4.57 190.12 0q-4.57 4.57-18.86 59.43-14.28 54.86 13.15 115.43-6.86 0-42.29 66.85t-16 206.28q29.72 129.14 117.14 176.57 87.43 47.43 167.43 44-2.28 1.14-19.43 25.14-17.14 24-16 68.58-1.15 2.28-36 14.28t-79.43-2.86q-43.43-20.57-65.71-49.71-22.28-29.14-22.28-31.43-1.15-5.71-36-34.28-34.86-28.58-79.43-12.58-3.43 0-1.14 12.58 2.28 12.57 46.86 34.28l26.29 40q22.86 36.57 38.28 69.71 15.43 33.14 73.14 50.86 57.72 17.72 129.72 1.72v133.71q1.14 1.14-2.29 6.85-3.43 5.72-20.57 10.86t-18.86 10.86q-1.72 5.72 7.43 6.86h418.28q9.14-1.14 7.43-6.86-1.71-5.72-18.86-10.86-17.14-5.14-20.58-10.86-3.43-5.71-2.28-6.85V762.28q-3.43-44.58-21.14-68.58-17.72-24-20-25.14 80 3.43 167.43-44t117.14-176.57q19.43-139.43-15.43-206.28t-41.72-66.85z" p-id="16650" fill="#1296db"></path></svg>
        </a>
      </div>

    </div>
    <div class="dao-footer-part3">
      Copyright@2021 www.rainbowdao.io
    </div>
  </div>
</template>

<script>
export default {
  name: "page-footer",
  methods:{
    navTo(){
      // window.open(url)
    }
  }
}
</script>

<style lang="scss" scoped>
$blehness:50%;
$radius: 10px;


@keyframes pulse_hue {
  0% {
    filter: hue-rotate(0deg) grayscale($blehness);
  }
  100% {
    filter: hue-rotate(360deg) grayscale($blehness);
  }
}

@keyframes twinkleWhite {
  0% {
    box-shadow: 0 0 0 #eee;
  }
  50% {
    box-shadow: 0px 0px 50px #eee;
  }
  100% {
    box-shadow: 0 0 0 #eee;
  }
}

.dao-footer {
  box-sizing: border-box;
  padding-bottom: 10px;
  background: url("../assets/daoImgs/footer_bg.png");
  background-size: 100%  100%;
  .line {
    width: 1100px;
    height: 1px;
    margin: 0px auto 0px;
    background: rgba(255, 255, 255, 0.10);
  }

  .dao-footer-part1 {
    width: 1000px;
    margin:  20px auto;
    display: flex;
    justify-content: space-between;
    flex-wrap: wrap;

    .footer-part2-item {
      position: relative;
      width: 220px;
      height: 80px;
      border-radius: 10px;
      margin: 14px auto;
      box-sizing: border-box;
      z-index: 1;

      &:nth-child(1) {
        --animate-duration: 200ms;
        --animate-delay: 0.1s;
      }

      &:nth-child(2) {
        --animate-duration: 250ms;
        --animate-delay: 0.15s;
      }

      &:nth-child(3) {
        --animate-duration: 300ms;
        --animate-delay: 0.2s;
      }

      &:nth-child(4) {
        --animate-duration: 350ms;
        --animate-delay: 0.25s;
      }

      &:nth-child(8) {
        --animate-duration: 400ms;
        --animate-delay: 0.3s;
      }

      &:nth-child(7) {
        --animate-duration: 450ms;
        --animate-delay: 0.35s;
      }

      &:nth-child(6) {
        --animate-duration: 500ms;
        --animate-delay: 0.4s !important;
      }

      &:nth-child(5) {
        --animate-duration: 550ms;
        --animate-delay: 0.45s !important;
      }

      .item-box {
        cursor: pointer;
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        position: relative;
        z-index: 2;
        border-radius: $radius;
        background: rgba(#1a1f5d,0.2);
        padding: 16px 15px;
      }
      &:hover{
        box-shadow: 0 0 18px #8f92fc;
        //animation: twinkleWhite 3s infinite;
      }

      .title {
        font-size: 18px;
        font-weight: 700;
        text-align: center;

        color: #fff;
        line-height: 21px;
      }

      .link {
        margin-top: 6px;
        color: #fff;
        text-align: center;
      }

      a {
        display: block;
        position: absolute;
        right: 10px;
        top: 24px;
        width: 26px;
        height: 26px;
        opacity: 1;
        background: rgba(255, 255, 255, 0.10);
        border: 1px solid rgba(255, 255, 255, 0.10);
        border-radius: 50%;
        line-height: 26px;
        text-align: center;

        img {
          width: 18px;
        }
      }
    }

    .footer-part2-item:nth-child(4n+1) {
      margin-left: 0;
    }
  }

  .dao-footer-part2 {
    width: 300px;
    margin: 0px auto 0;
    display: flex;
    justify-content: space-between;

    .icon-box {
      cursor: pointer;
      width: 30px;
      height: 30px;
      background: rgba(255,255,255,0.12);
      border-radius: 50%;
      display: flex;
      justify-content: center;
      align-items: center;
      position: relative;
      padding: 10px;

      .icon path{
        fill: white!important;
      }
      &:hover .icon path{
        fill: #10101B!important;
      }
      span {
        width: 0%;
        transition: 0.3s;
        height: 0%;
        border-radius: 50%;
        position: absolute;
        margin: 3px;
        background: white;
      }

      &:hover span {
        width: 100%;
        height: 100%;
        margin: 0px;
      }
      .icon {
        -webkit-transition: all 0.3s;
        -moz-transition: all 0.3s;
        -o-transition: all 0.3s;
        transition: all 0.3s;
        width: 18px;
        height: 18px;
        position: relative;
      }

    }
  }

  .dao-footer-part3 {
    margin: 30px 0;
    color: white;
    text-align: center;
  }
}
</style>
