import {UploadOutlined} from '@ant-design/icons';
import type {UploadProps} from 'antd';
import {Button, Upload, message} from 'antd';
import {useEffect} from 'react';
import './App.css';
// import {greet} from './pkg/dai_wasm?init';


function App() {
  // const open = () => {
  //   greet("word")
  // }

  const props: UploadProps = {
    name: 'file',
    action: 'https://www.mocky.io/v2/5cc8019d300000980a055e76',
    headers: {
      authorization: 'authorization-text',
    },
    onChange(info) {
      if (info.file.status !== 'uploading') {
        console.log(info.file, info.fileList);
      }
      if (info.file.status === 'done') {
        message.success(`${info.file.name} file uploaded successfully`);
      } else if (info.file.status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
  };

  useEffect(() => {
    try {
      import('@silvia-odwyer/photon').then((module) => {
        console.log(module.resize)
      })
    } catch (e) {
      console.log(e)
    }
  }, [])


  return (
    <div>
      <Upload {...props}>
        <Button icon={<UploadOutlined />}>Click to Upload</Button>
      </Upload>
    </div>
  )
}

export default App
