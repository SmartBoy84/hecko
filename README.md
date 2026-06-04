# hecko
WIP lecture viewer for echo360. 

# Idea scratch board
- Cache videos in the background
- Allow downloading
- Chrome extension to intercept visits to echo360 and resirect to app
    - Encode token in uri (or maybe local protocol, websockets?)
- Allow splitting streams into separate screens
- Allow downloading
- MUST be native in rust
- Ideally get login to work so I can view watch history
- Allow searching for keywords in lectures to find lectures
- Allow tagging lectures or adding notes to lectures

Session features
- Show watched lectures
- Automatically indicate lectures with no audio… Not sure how I’d do this - maybe snapshot at different times? Check if same frame or audio is just white noise
- Use a binary search type algorithm to determine *actual* length of video - ie check for non-white noise at half way etc
- Allow adding markdown notes for lectures
- Bookmark lectures
- Select favourite units
- Denoise lectures and enhance audio
- Search using sound to account for accent
    - E.g., lu jin says “lecture” → add it to a database, mark it as ‘lecture’ then search using that!
- Inter-op with note taking apps
    - Allow linking to specific times, in specific lectures (like on YouTube)
- Store config on echo by uploading a video
    - Add a hotkey to quickly copy a markdown link
- Can store config data using echo itself (videos allow storing notes)