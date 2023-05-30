// This file has been generated by Specta. DO NOT EDIT.

export type BixboxData = { name: string; image: string; worst_rating: number; best_rating: number; rating_count: number; rating_value: number; description: string; author: string; date_published: string; date_modified: string; title: string; genres: MgenTag[] }

export type ReaderAreaImage = { url: string; width: number; height: number; decoding: string; alt: string }

export type Status = "All" | "Ongoing" | "Completed" | "Hiatus"

export type MgenTag = { url: string; name: string }

export type RawKumaChapterData = { reader_area: ReaderArea; related_mangas: BsxTitleData[] }

export type RawKumaMangaDetailData = { data: BixboxData; chapterlist: ChapterList; related_series: BsxTitleData[] }

export type ChapterList = { chapters: Chapter[] }

export type UtaoTitleChapter = { url: string; text: string }

export type UtaoTitleData = { title: string; url: string; image: string; chapters: UtaoTitleChapter[] }

export type Chapter = { url: string; chapter_num: string; chapter_date: string; num: number; download_link: string }

export type BsxTitleData = { title: string; url: string; image: string; rating: number }

export type Type = "All" | "Manga" | "Manhwa" | "Manhua" | "Comic" | "Novel"

export type MangaListParameter = { page: number; status: Status; genre: Genre[]; order: Order; type: Type }

export type RawKumaSearch = { result: BsxTitleData[] }

export type Order = "Default" | "AZ" | "ZA" | "Update" | "Added" | "Popular"

export type RawKumaHomeData = { popular_title: BsxTitleData[]; recommandation: { [key: string]: BsxTitleData[] }; latest_update: UtaoTitleData[] }

export type ReaderArea = { images: ReaderAreaImage[] }

export type Genre = "All" | "Action" | "Adult" | "Adventure" | "Alternative_World" | "Comedy" | "Drama" | "Ecchi" | "Elves" | "English" | "Fantasy" | "Food" | "Game" | "Gender_Bender" | "Harem" | "Historical" | "Horror" | "Isekai" | "Josei" | "Lolicon" | "Magic" | "Martial_Arts" | "Mature" | "Mecha" | "Medical" | "Mystery" | "N_A" | "Oneshot" | "Psychological" | "Romance" | "School_Life" | "Sci_Fi" | "Seinen" | "Shotacon" | "Shoujo" | "Shoujo_Ai" | "Shounen" | "Shounen_Ai" | "Slice_Of_Life" | "Smut" | "Sports" | "Supernatural" | "Tragedy" | "Updating" | "War" | "Yaoi" | "Yuri"

