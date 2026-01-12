<script setup lang="tsx">
import {getUrlParams} from "../../utils/navigation";

const windowLocation = `${window.location.origin}${window.location.pathname}`;

type TargetUrl = {
  name: string;
  url: string;
};

const x86_64UnknownLinuxGnu = {
  name: "x86_64-unknown-linux-gnu",
  url: `${windowLocation}?target=x86_64-unknown-linux-gnu`,
};

const AArch64UnknownLinuxGnu = {
  name: "aarch64-unknown-linux-gnu",
  url: `${windowLocation}?target=aarch64-unknown-linux-gnu`,
};

const TargetUrls = [x86_64UnknownLinuxGnu, AArch64UnknownLinuxGnu];

function getActiveClass(target: TargetUrl): string {
  const params = getUrlParams();
  const curTarget = params?.["target"];
  if (!curTarget) {
    return target.name === x86_64UnknownLinuxGnu.name ? "target-active" : "";
  }
  return target.name == curTarget ? "target-active" : "";
}
</script>

<template>
  <div class="target-wrapper">
    <strong>Targets: </strong>
    <div class="target-list-wrapper">
      <template v-for="target in TargetUrls">
        <span class="target-list-element">
          <a :class="getActiveClass(target)" :href="target.url"
            >{{ target.name }}
          </a>
        </span>
      </template>
    </div>
  </div>
</template>

<style scoped lang="scss">
.target-wrapper {
  padding-top: 5px;
  display: flex;
  flex-direction: column;
}

.target-list-wrapper {
  display: flex;
}

.target-active {
  font-weight: bold;
  text-decoration: underline;
}

.target-list-element {
  padding-right: 5px;
}
</style>
