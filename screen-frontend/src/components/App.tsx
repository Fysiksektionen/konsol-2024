import { useEffect, useState } from 'react'

import '../index.css';
import '../styles/App.css';

import { SlideData } from '../types/slides/SlideData.ts'

import SlData from "../types/sl/SlData.ts";

import { LayoutType, ColorMode } from '../types/settings/settings-types.ts'
import { Settings } from '../types/settings/Settings.ts';
import FullscreenSlideshowLayout from './layouts/FullscreenSlideshowLayout.tsx';
import MixedLayout from './layouts/MixedLayout.tsx';

function App() {
  const BASE_URL = 'http://localhost:8080'; // replace with 'http://f.kth.se/konsol'

  const [slides, setSlides] = useState<SlideData[]>([]);

  const [sl, setSl] = useState<{ data: SlData }>({data: new SlData([
        { site_id: 9204, tracked_lines: undefined }, // tekniska högskolan
        { site_id: 9600, tracked_lines: undefined }, // östra station
        // { site_id: 1080, tracked_lines: undefined }  // stockholm city
    ])});

  const [settings, setSettings] = useState<Settings>(new Settings(
    LayoutType.Mixed, 
    ColorMode.DarkMode
  ));

  useEffect(() => {
    fetch(`${BASE_URL}/api/screen/slides`)
      .then(response => response.json())
      .then(data => {
        setSlides(data);
        console.log(data);
      })
      .catch(error => {
        console.error('Error fetching slides:', error)
      })
  }, []);

  useEffect(() => {
    const updateSlData = () => sl.data.update().then(() => {
        setSl({data: sl.data});
      });
    const handle = setInterval(updateSlData, 5 * 1000)
    updateSlData();
    return () => clearInterval(handle);
  }, []);

  useEffect(() => {
    fetch(`${BASE_URL}/api/screen/settings`)
      .then(response => response.json())
      .then(data => {
        setSettings(data);
      })
      .catch(error => {
        console.error('Error fetching settings:', error)
      })
  }, []);

  return <> {
    settings.layout_type === LayoutType.FullscreenSlideshow ? 
      <FullscreenSlideshowLayout slides={slides}/> :
    settings.layout_type === LayoutType.Mixed ?
      <MixedLayout slides={slides} sl_data={sl.data}/> :
    (() => {throw new Error("Invalid layout type");})()
  } </>;
}

export default App
