<template>
  <div class="tooltip">
    ?
    <span class="tooltiptext">
      <slot></slot>
    </span>
  </div>
</template>

<style scoped lang="scss">
.tooltip {
  position: relative;
  border-radius: 50%;
  font-size: 12px;
  margin: 0 4px;
  border: 1px solid #7d6969;
  width: 16px;
  text-align: center;
  font-weight: bold;
  background: #d6d3d3;
  cursor: pointer;
  display: inline-block;
}

.tooltip .tooltiptext {
  width: 180px;
  visibility: hidden;
  color: white;
  background-color: #524d4d;
  text-align: center;
  padding: 5px;
  border-radius: 6px;

  position: absolute;
  bottom: 125%;
  margin-left: -60px;

  opacity: 0;
  transition: opacity 0.3s, visibility 0.3s;
}

.tooltip:hover .tooltiptext {
  visibility: visible;
  opacity: 1;
}
</style>
