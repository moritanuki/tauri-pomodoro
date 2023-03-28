<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

let clear: NodeJS.Timer;
const defaultRunMin = 1;
const defaultRunSec = 10;
const defaultRestMin = 0;
const defaultRestSec = 10;

const min = ref(defaultRunMin);
const sec = ref(defaultRunSec);
const status = ref("Run");
const isActive = ref(false);

const timer = () => {
  if (min.value === 0 && sec.value === 0) {
    if (status.value === "Run") {
      status.value = "Rest";
      min.value = defaultRestMin;
      sec.value = defaultRestSec;
    } else {
      status.value = "Run";
      min.value = defaultRunMin;
      sec.value = defaultRunSec;
    }
  } else if (sec.value === 0) {
    sec.value = 59;
    min.value = (min.value - 1);
  } else {
    sec.value = (sec.value - 1);
  }
};

const timeToStr = (time: Number) => {
  return time.toString().padStart(2, "0");
};

const createWindow = async () => {
  await invoke("create_window");
};

const start = () => {
  clear = setInterval(timer, 1000);
  isActive.value = true;
};

const pause = () => {
  clearInterval(clear);
  isActive.value = false;
  createWindow();
};

const reset = () => {
  clearInterval(clear);
  min.value = defaultRunMin;
  sec.value = defaultRunSec;
  status.value = "Run";
  isActive.value = false;
};

if (status.value === "Run") {
  start();
}

</script>

<template>
  <h3 id="timer">{{ `${timeToStr(min)}:${timeToStr(sec)}` }}</h3>

  <div :class="{'status': true, 'start-color': isActive}">{{ status }}</div>

  <div class="button-wrap">
    <button @click="start">Start</button>
    <button @click="pause">Pause</button>
    <button @click="reset">Reset</button>
  </div>
</template>

<style scoped>

#timer {
  font-size: 2rem;
  font-weight: 700;
  color: #555;
  text-align: center;
  margin: 20px 0;
}

.status {
  font-size: 0.8rem;
  font-weight: 600;
  color: #999;
  text-align: center;
}

.start-color {
  color: #444;
}

.button-wrap {
  display: flex;
  justify-content: center;
}

button {
  display: block;
  border-radius: 8px;
  background-color: #aaa;
  color: #fff;
  font-size: 0.7rem;
  font-weight: 600;
  border: none;
  margin: 0 auto;
}

</style>
