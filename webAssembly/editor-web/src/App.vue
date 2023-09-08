<script setup lang="ts">
import axios from 'axios';
import { NButton, NInput, NSpace, NUpload, UploadCustomRequestOptions } from 'naive-ui';
import { ref } from 'vue';

const size = ref(15);
const file_name = ref("");

// methods
const upload = ({file}: UploadCustomRequestOptions) => {
  file_name.value = file.name


  const formdata = new FormData()
  formdata.append('file', file.file as File)
  axios.post('/api/upload', formdata, {
    headers: {
      'Content-Type': 'multipart/form-data'
    }
  }).then(res => {
    console.log(res)
  }).catch(err => {
    console.log(err)
  })
}

const open = () => {
    window.open(`/api/img?filename=${file_name.value}&size=${size.value}`, '_blank')
}

</script>

<template>
  <div>
    <n-space vertical>
      <div>
        {{ file_name || "" }}
      </div>
      <n-upload accept=".png, .jpg, .jpeg" :custom-request="upload">
        <n-button>上传文件</n-button>
      </n-upload>
      <n-space>
        <n-input v-model:value="size"  placeholder="尺寸" />
        <n-button @click="open">
          下载
        </n-button>
      </n-space>
    </n-space>
  </div>
</template>

<style scoped>
</style>
