import React from 'react';
import { Layout, Menu, theme } from 'antd';
import {PageActors} from "./pages/Actors.tsx";
const { Header, Content, Footer, Sider } = Layout;

const App: React.FC = () => {
    const {
        token: { colorBgContainer },
    } = theme.useToken();

    return (
        <Layout hasSider>
            <Sider
                style={{
                    overflow: 'auto',
                    height: '100vh',
                    position: 'fixed',
                    left: 0,
                    top: 0,
                    bottom: 0,
                }}
            >
                <div className="demo-logo-vertical" />
                <Menu theme="dark" mode="inline" defaultSelectedKeys={['4']} items={[
                    {
                        key: 1,
                        label: "Processes",
                    }
                ]} />
            </Sider>
            <Layout className="site-layout" style={{ marginLeft: 200 }}>
                <Header style={{ padding: 0, background: colorBgContainer }} />
                <Content style={{ margin: '24px 16px 0', overflow: 'initial' }}>
                    <div style={{ padding: 24, textAlign: 'center', background: colorBgContainer }}>
                        <PageActors/>
                    </div>
                </Content>
            </Layout>
        </Layout>
    );
};

export default App;