import { useEffect, useState } from 'react';
import '../styles/Calendar.css';

interface CalendarEvent {
    id: string;
    summary: string;
    start: {
        dateTime?: string;
        date?: string;
    };
    end: {
        dateTime?: string;
        date?: string;
    };
    location?: string;
}

interface CalendarProps {
    apiKey?: string;
    calendarId: string;
}

export default function Calendar({ apiKey, calendarId }: CalendarProps) {
    const [events, setEvents] = useState<CalendarEvent[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (!apiKey) {
            // Mock data for development if no API key is provided
            setEvents([
                {
                    id: '1',
                    summary: 'Pub',
                    start: { dateTime: new Date(Date.now() + 3600000).toISOString() },
                    end: { dateTime: new Date(Date.now() + 7200000).toISOString() },
                    location: 'Konsulatet',
                },
                {
                    id: '2',
                    summary: 'Sektionsmöte',
                    start: { dateTime: new Date(Date.now() + 86400000).toISOString() },
                    end: { dateTime: new Date(Date.now() + 90000000).toISOString() },
                    location: 'T-Centralen',
                },
                {
                    id: '3',
                    summary: 'Tentamen i Kvantfysik',
                    start: { date: new Date(Date.now() + 172800000).toISOString().split('T')[0] },
                    end: { date: new Date(Date.now() + 172800000).toISOString().split('T')[0] },
                    location: 'FA32',
                }
            ]);
            setLoading(false);
            return;
        }

        const fetchEvents = async () => {
            try {
                const now = new Date().toISOString();
                const response = await fetch(
                    `https://www.googleapis.com/calendar/v3/calendars/${encodeURIComponent(calendarId)}/events?key=${apiKey}&timeMin=${now}&singleEvents=true&orderBy=startTime&maxResults=10`
                );

                if (!response.ok) {
                    throw new Error('Failed to fetch events');
                }

                const data = await response.json();
                setEvents(data.items || []);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'Unknown error');
                console.error('Calendar error:', err);
            } finally {
                setLoading(false);
            }
        };

        fetchEvents();
        const interval = setInterval(fetchEvents, 600000); // Refresh every 10 minutes (600,000 ms)

        return () => clearInterval(interval);
    }, [apiKey, calendarId]);

    if (loading) return <div className="calendar-loading">Laddar kalender...</div>;
    if (error) return <div className="calendar-error">Kunde inte hämta kalender</div>;

    const formatDate = (dateStr?: string) => {
        if (!dateStr) return '';
        const date = new Date(dateStr);
        return date.toLocaleDateString('sv-SE', { weekday: 'short', day: 'numeric', month: 'short' });
    };

    const formatTime = (dateStr?: string) => {
        if (!dateStr) return '';
        const date = new Date(dateStr);
        return date.toLocaleTimeString('sv-SE', { hour: '2-digit', minute: '2-digit' });
    };

    return (
        <div className="calendar-component">
            <h2 className="calendar-title">Kalender</h2>
            <div className="events-list">
                {events.length === 0 ? (
                    <p className="no-events">Inga kommande händelser</p>
                ) : (
                    events.map((event) => {
                        const isAllDay = !!event.start.date;
                        const startDate = event.start.dateTime || event.start.date;
                        return (
                            <div key={event.id} className="event-card">
                                <div className="event-date">
                                    <span className="date-main">{formatDate(startDate)}</span>
                                    {!isAllDay && <span className="event-time">{formatTime(event.start.dateTime)}</span>}
                                </div>
                                <div className="event-details">
                                    <h3 className="event-summary">{event.summary}</h3>
                                    {event.location && <p className="event-location">{event.location}</p>}
                                </div>
                            </div>
                        );
                    })
                )}
            </div>
        </div>
    );
}
