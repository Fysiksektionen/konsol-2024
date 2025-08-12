import React from 'react';
import { SlideData } from '../types';
import '../styles/Slide.css';
import { updateSlides } from '../util/utils';

type Props = {
    slide: SlideData;
    setSlides: (slides: SlideData[]) => void;
}

const Slide = ({ slide, setSlides }: Props) => {
    const handleRemove = () => {
        fetch(`${import.meta.env.VITE_API_BASE_URL}/api/screen/slides/${slide.id}`, {
            method: 'DELETE'
        }).then(response => {
            if (response.ok) {
                console.log('Slide removed successfully');
                updateSlides(setSlides); // Refresh slides
            } else {
                console.log('Failed to remove slide');
            }
        });
    }

    const truncateCaption = (caption: string) => {
        return caption.length > 30 ? caption.substring(0, 30) + '...' : caption;
    }
    
    return (
        <div className='slide'>
            <div className={`indicator ${slide.active ? 'active' : 'inactive'}`} />
            <h2>{ truncateCaption(slide.caption) }</h2>
            <img className='slide-image' src={`${import.meta.env.VITE_API_BASE_URL}/api/screen/slides/images/${slide.id}.${slide.filetype}`} alt={slide.caption} />  {/*  http://localhost:8080/api/slides/${slide.id} */}
            <p>
                {new Date(slide.start_date).toLocaleString()} &ndash; {new Date(slide.end_date).toLocaleString()}
            </p>
            <button className='remove-button' onClick={handleRemove}>X</button>
        </div>
    )
};

export default Slide;