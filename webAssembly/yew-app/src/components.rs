use crate::Video;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideoList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            html! {
            <p key={video.id}>
            {
                format!("{}: {}", video.speaker, video.title)
            }
              <a href={video.url.clone()} target={"_blank"}>
              {video.url.clone()}
                  </a>
                </p>
            }
        })
        .collect()
}
