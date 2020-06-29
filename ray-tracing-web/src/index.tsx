import React, { useEffect, useState, useRef, Suspense } from "react";
import ReactDOM from "react-dom";
import JsRayTracing from "./pure-js";
import RustRayTracing from "./rust-wasm";
import SmallRustRayTracing from './rust-wasm/small'
import "antd/dist/antd.css";
import "./index.css";

import { Layout, Menu } from "antd";

const { Header, Content, Footer } = Layout;

const Components = {
  JavaScript: JsRayTracing,
  Rust: RustRayTracing,
  ['Rust: Small image']: SmallRustRayTracing
};

type Languages = keyof typeof Components;

const App = () => {
  let [type, setType] = useState<Languages>("Rust");
  let [counts, setCounts] = useState(100);

  let Target = Components[type] || JsRayTracing;

  let handleCountChange: React.ChangeEventHandler<HTMLInputElement> = (
    event
  ) => {
    setCounts(parseInt(event.target.value, 10));
  };

  return (
    <Layout className='layout'>
      <Header>
        <div className='logo' />
        <Menu theme='dark' mode='horizontal' selectedKeys={[type]}>
          {Object.keys(Components).map((name) => {
            let handleClick = () => {
              setType(name as Languages);
            };
            return (
              <Menu.Item key={name} onClick={handleClick}>
                {name}
              </Menu.Item>
            );
          })}
        </Menu>
      </Header>
      <Content style={{ padding: "0 250px" }}>
        <div className='site-layout-content'>
          <Suspense fallback='loading...'>
            <Target counts={counts} />
          </Suspense>
        </div>
      </Content>
      <Footer style={{ textAlign: "center" }}>
        <a href='https://github.com/Lucifier129/rust-ray-tracing-demo'>
          Ray Tracing Demos Created by 工业聚
        </a>
      </Footer>
    </Layout>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
