import React, {lazy as PageLazyLoad} from "react";
import { BrowserRouter, Route, Routes as Switch, Navigate } from "react-router-dom";


import AppLoader from "./components/layout/AppLoader";

const Home = PageLazyLoad(() => import("./screens/Home"));
const NotFound = PageLazyLoad(() => import("./screens/NotFound"));


const EL = (props: any) => {
  const { as: Component } = props;
  return <Component />;
};

const Routes = () => {
  const routes = [
    { component: Home, path: "/"},
  ];

  // Fallback route
  routes.push({ component: NotFound, path: "*"});

  return (
    <React.Suspense fallback={<AppLoader />}>
      <BrowserRouter>
        <Switch>
          {routes.map((route, key) => {
            return  <Route key={key} path={route.path} element={<EL as={route.component} />} />
          })}
        </Switch>
      </BrowserRouter>
    </React.Suspense>
  );
};

export default Routes;
