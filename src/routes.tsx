import React, {lazy as PageLazyLoad} from "react";
import { BrowserRouter, Route, Routes as Switch, Navigate } from "react-router-dom";
import { changeWindowTitle } from "./commands";


import AppLoader from "./components/layout/AppLoader";

const Home = PageLazyLoad(() => import("./screens/Home"));
const PreferencesScreen = PageLazyLoad(() => import("./screens/preferences"));
const NotFound = PageLazyLoad(() => import("./screens/NotFound"));


const EL = (props: any) => {
  const { as: Component,title } = props;
  changeWindowTitle(title); // change window title on navigation
  return <Component />;
};

const Routes = () => {
  const routes = [
    { component: Home, path: "/", title:"{dev.all.day}"},
    { component: PreferencesScreen, path: "/preferences", title:"Preferences"},
  ];

  // Fallback route
  routes.push({ component: NotFound, path: "*",title: "Error: Not Found!"});

  return (
    <React.Suspense fallback={<AppLoader />}>
      <BrowserRouter>
        <Switch>
          {routes.map((route, key) => {
            return  <Route key={key} path={route.path} element={<EL as={route.component} title={route.title} />} />
          })}
        </Switch>
      </BrowserRouter>
    </React.Suspense>
  );
};

export default Routes;
