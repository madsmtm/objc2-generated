//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// CSMedia.
#[cfg(feature = "CSSearchableItemAttributeSet")]
impl CSSearchableItemAttributeSet {
    extern_methods!(
        #[unsafe(method(editors))]
        #[unsafe(method_family = none)]
        pub unsafe fn editors(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`editors`][Self::editors].
        #[unsafe(method(setEditors:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEditors(&self, editors: Option<&NSArray<NSString>>);

        #[unsafe(method(participants))]
        #[unsafe(method_family = none)]
        pub unsafe fn participants(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`participants`][Self::participants].
        #[unsafe(method(setParticipants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setParticipants(&self, participants: Option<&NSArray<NSString>>);

        #[unsafe(method(projects))]
        #[unsafe(method_family = none)]
        pub unsafe fn projects(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`projects`][Self::projects].
        #[unsafe(method(setProjects:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setProjects(&self, projects: Option<&NSArray<NSString>>);

        #[unsafe(method(downloadedDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn downloadedDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`downloadedDate`][Self::downloadedDate].
        #[unsafe(method(setDownloadedDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDownloadedDate(&self, downloaded_date: Option<&NSDate>);

        #[unsafe(method(contentSources))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentSources(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`contentSources`][Self::contentSources].
        #[unsafe(method(setContentSources:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentSources(&self, content_sources: Option<&NSArray<NSString>>);

        #[unsafe(method(comment))]
        #[unsafe(method_family = none)]
        pub unsafe fn comment(&self) -> Option<Retained<NSString>>;

        /// Setter for [`comment`][Self::comment].
        #[unsafe(method(setComment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setComment(&self, comment: Option<&NSString>);

        #[unsafe(method(copyright))]
        #[unsafe(method_family = none)]
        pub unsafe fn copyright(&self) -> Option<Retained<NSString>>;

        /// Setter for [`copyright`][Self::copyright].
        #[unsafe(method(setCopyright:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCopyright(&self, copyright: Option<&NSString>);

        #[unsafe(method(lastUsedDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn lastUsedDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`lastUsedDate`][Self::lastUsedDate].
        #[unsafe(method(setLastUsedDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLastUsedDate(&self, last_used_date: Option<&NSDate>);

        #[unsafe(method(contentCreationDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentCreationDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`contentCreationDate`][Self::contentCreationDate].
        #[unsafe(method(setContentCreationDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentCreationDate(&self, content_creation_date: Option<&NSDate>);

        #[unsafe(method(contentModificationDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentModificationDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`contentModificationDate`][Self::contentModificationDate].
        #[unsafe(method(setContentModificationDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentModificationDate(&self, content_modification_date: Option<&NSDate>);

        #[unsafe(method(addedDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn addedDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`addedDate`][Self::addedDate].
        #[unsafe(method(setAddedDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAddedDate(&self, added_date: Option<&NSDate>);

        #[unsafe(method(duration))]
        #[unsafe(method_family = none)]
        pub unsafe fn duration(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`duration`][Self::duration].
        #[unsafe(method(setDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDuration(&self, duration: Option<&NSNumber>);

        #[unsafe(method(contactKeywords))]
        #[unsafe(method_family = none)]
        pub unsafe fn contactKeywords(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`contactKeywords`][Self::contactKeywords].
        #[unsafe(method(setContactKeywords:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContactKeywords(&self, contact_keywords: Option<&NSArray<NSString>>);

        #[unsafe(method(codecs))]
        #[unsafe(method_family = none)]
        pub unsafe fn codecs(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`codecs`][Self::codecs].
        #[unsafe(method(setCodecs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCodecs(&self, codecs: Option<&NSArray<NSString>>);

        #[unsafe(method(mediaTypes))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaTypes(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`mediaTypes`][Self::mediaTypes].
        #[unsafe(method(setMediaTypes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMediaTypes(&self, media_types: Option<&NSArray<NSString>>);

        #[unsafe(method(isStreamable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isStreamable(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`isStreamable`][Self::isStreamable].
        #[unsafe(method(setStreamable:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStreamable(&self, streamable: Option<&NSNumber>);

        #[unsafe(method(totalBitRate))]
        #[unsafe(method_family = none)]
        pub unsafe fn totalBitRate(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`totalBitRate`][Self::totalBitRate].
        #[unsafe(method(setTotalBitRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTotalBitRate(&self, total_bit_rate: Option<&NSNumber>);

        #[unsafe(method(videoBitRate))]
        #[unsafe(method_family = none)]
        pub unsafe fn videoBitRate(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`videoBitRate`][Self::videoBitRate].
        #[unsafe(method(setVideoBitRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVideoBitRate(&self, video_bit_rate: Option<&NSNumber>);

        #[unsafe(method(audioBitRate))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioBitRate(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`audioBitRate`][Self::audioBitRate].
        #[unsafe(method(setAudioBitRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioBitRate(&self, audio_bit_rate: Option<&NSNumber>);

        #[unsafe(method(deliveryType))]
        #[unsafe(method_family = none)]
        pub unsafe fn deliveryType(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`deliveryType`][Self::deliveryType].
        #[unsafe(method(setDeliveryType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDeliveryType(&self, delivery_type: Option<&NSNumber>);

        #[unsafe(method(organizations))]
        #[unsafe(method_family = none)]
        pub unsafe fn organizations(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`organizations`][Self::organizations].
        #[unsafe(method(setOrganizations:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOrganizations(&self, organizations: Option<&NSArray<NSString>>);

        #[unsafe(method(role))]
        #[unsafe(method_family = none)]
        pub unsafe fn role(&self) -> Option<Retained<NSString>>;

        /// Setter for [`role`][Self::role].
        #[unsafe(method(setRole:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRole(&self, role: Option<&NSString>);

        #[unsafe(method(languages))]
        #[unsafe(method_family = none)]
        pub unsafe fn languages(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`languages`][Self::languages].
        #[unsafe(method(setLanguages:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLanguages(&self, languages: Option<&NSArray<NSString>>);

        #[unsafe(method(rights))]
        #[unsafe(method_family = none)]
        pub unsafe fn rights(&self) -> Option<Retained<NSString>>;

        /// Setter for [`rights`][Self::rights].
        #[unsafe(method(setRights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRights(&self, rights: Option<&NSString>);

        #[unsafe(method(publishers))]
        #[unsafe(method_family = none)]
        pub unsafe fn publishers(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`publishers`][Self::publishers].
        #[unsafe(method(setPublishers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPublishers(&self, publishers: Option<&NSArray<NSString>>);

        #[unsafe(method(contributors))]
        #[unsafe(method_family = none)]
        pub unsafe fn contributors(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`contributors`][Self::contributors].
        #[unsafe(method(setContributors:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContributors(&self, contributors: Option<&NSArray<NSString>>);

        #[unsafe(method(coverage))]
        #[unsafe(method_family = none)]
        pub unsafe fn coverage(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`coverage`][Self::coverage].
        #[unsafe(method(setCoverage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCoverage(&self, coverage: Option<&NSArray<NSString>>);

        #[unsafe(method(rating))]
        #[unsafe(method_family = none)]
        pub unsafe fn rating(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`rating`][Self::rating].
        #[unsafe(method(setRating:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRating(&self, rating: Option<&NSNumber>);

        #[unsafe(method(ratingDescription))]
        #[unsafe(method_family = none)]
        pub unsafe fn ratingDescription(&self) -> Option<Retained<NSString>>;

        /// Setter for [`ratingDescription`][Self::ratingDescription].
        #[unsafe(method(setRatingDescription:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRatingDescription(&self, rating_description: Option<&NSString>);

        #[unsafe(method(playCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn playCount(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`playCount`][Self::playCount].
        #[unsafe(method(setPlayCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPlayCount(&self, play_count: Option<&NSNumber>);

        #[unsafe(method(information))]
        #[unsafe(method_family = none)]
        pub unsafe fn information(&self) -> Option<Retained<NSString>>;

        /// Setter for [`information`][Self::information].
        #[unsafe(method(setInformation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInformation(&self, information: Option<&NSString>);

        #[unsafe(method(director))]
        #[unsafe(method_family = none)]
        pub unsafe fn director(&self) -> Option<Retained<NSString>>;

        /// Setter for [`director`][Self::director].
        #[unsafe(method(setDirector:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDirector(&self, director: Option<&NSString>);

        #[unsafe(method(producer))]
        #[unsafe(method_family = none)]
        pub unsafe fn producer(&self) -> Option<Retained<NSString>>;

        /// Setter for [`producer`][Self::producer].
        #[unsafe(method(setProducer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setProducer(&self, producer: Option<&NSString>);

        #[unsafe(method(genre))]
        #[unsafe(method_family = none)]
        pub unsafe fn genre(&self) -> Option<Retained<NSString>>;

        /// Setter for [`genre`][Self::genre].
        #[unsafe(method(setGenre:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGenre(&self, genre: Option<&NSString>);

        #[unsafe(method(performers))]
        #[unsafe(method_family = none)]
        pub unsafe fn performers(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`performers`][Self::performers].
        #[unsafe(method(setPerformers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerformers(&self, performers: Option<&NSArray<NSString>>);

        #[unsafe(method(originalFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn originalFormat(&self) -> Option<Retained<NSString>>;

        /// Setter for [`originalFormat`][Self::originalFormat].
        #[unsafe(method(setOriginalFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOriginalFormat(&self, original_format: Option<&NSString>);

        #[unsafe(method(originalSource))]
        #[unsafe(method_family = none)]
        pub unsafe fn originalSource(&self) -> Option<Retained<NSString>>;

        /// Setter for [`originalSource`][Self::originalSource].
        #[unsafe(method(setOriginalSource:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOriginalSource(&self, original_source: Option<&NSString>);

        #[unsafe(method(isLocal))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLocal(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`isLocal`][Self::isLocal].
        #[unsafe(method(setLocal:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocal(&self, local: Option<&NSNumber>);

        #[unsafe(method(contentRating))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentRating(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`contentRating`][Self::contentRating].
        #[unsafe(method(setContentRating:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setContentRating(&self, content_rating: Option<&NSNumber>);

        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        /// Setter for [`URL`][Self::URL].
        #[unsafe(method(setURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);
    );
}

/// CSMusic.
#[cfg(feature = "CSSearchableItemAttributeSet")]
impl CSSearchableItemAttributeSet {
    extern_methods!(
        #[unsafe(method(audioSampleRate))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioSampleRate(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`audioSampleRate`][Self::audioSampleRate].
        #[unsafe(method(setAudioSampleRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioSampleRate(&self, audio_sample_rate: Option<&NSNumber>);

        #[unsafe(method(audioChannelCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioChannelCount(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`audioChannelCount`][Self::audioChannelCount].
        #[unsafe(method(setAudioChannelCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioChannelCount(&self, audio_channel_count: Option<&NSNumber>);

        #[unsafe(method(tempo))]
        #[unsafe(method_family = none)]
        pub unsafe fn tempo(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`tempo`][Self::tempo].
        #[unsafe(method(setTempo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTempo(&self, tempo: Option<&NSNumber>);

        #[unsafe(method(keySignature))]
        #[unsafe(method_family = none)]
        pub unsafe fn keySignature(&self) -> Option<Retained<NSString>>;

        /// Setter for [`keySignature`][Self::keySignature].
        #[unsafe(method(setKeySignature:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setKeySignature(&self, key_signature: Option<&NSString>);

        #[unsafe(method(timeSignature))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeSignature(&self) -> Option<Retained<NSString>>;

        /// Setter for [`timeSignature`][Self::timeSignature].
        #[unsafe(method(setTimeSignature:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTimeSignature(&self, time_signature: Option<&NSString>);

        #[unsafe(method(audioEncodingApplication))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioEncodingApplication(&self) -> Option<Retained<NSString>>;

        /// Setter for [`audioEncodingApplication`][Self::audioEncodingApplication].
        #[unsafe(method(setAudioEncodingApplication:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioEncodingApplication(
            &self,
            audio_encoding_application: Option<&NSString>,
        );

        #[unsafe(method(composer))]
        #[unsafe(method_family = none)]
        pub unsafe fn composer(&self) -> Option<Retained<NSString>>;

        /// Setter for [`composer`][Self::composer].
        #[unsafe(method(setComposer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setComposer(&self, composer: Option<&NSString>);

        #[unsafe(method(lyricist))]
        #[unsafe(method_family = none)]
        pub unsafe fn lyricist(&self) -> Option<Retained<NSString>>;

        /// Setter for [`lyricist`][Self::lyricist].
        #[unsafe(method(setLyricist:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLyricist(&self, lyricist: Option<&NSString>);

        #[unsafe(method(album))]
        #[unsafe(method_family = none)]
        pub unsafe fn album(&self) -> Option<Retained<NSString>>;

        /// Setter for [`album`][Self::album].
        #[unsafe(method(setAlbum:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAlbum(&self, album: Option<&NSString>);

        #[unsafe(method(artist))]
        #[unsafe(method_family = none)]
        pub unsafe fn artist(&self) -> Option<Retained<NSString>>;

        /// Setter for [`artist`][Self::artist].
        #[unsafe(method(setArtist:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setArtist(&self, artist: Option<&NSString>);

        #[unsafe(method(audioTrackNumber))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioTrackNumber(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`audioTrackNumber`][Self::audioTrackNumber].
        #[unsafe(method(setAudioTrackNumber:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioTrackNumber(&self, audio_track_number: Option<&NSNumber>);

        #[unsafe(method(recordingDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordingDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`recordingDate`][Self::recordingDate].
        #[unsafe(method(setRecordingDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRecordingDate(&self, recording_date: Option<&NSDate>);

        #[unsafe(method(musicalGenre))]
        #[unsafe(method_family = none)]
        pub unsafe fn musicalGenre(&self) -> Option<Retained<NSString>>;

        /// Setter for [`musicalGenre`][Self::musicalGenre].
        #[unsafe(method(setMusicalGenre:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMusicalGenre(&self, musical_genre: Option<&NSString>);

        #[unsafe(method(isGeneralMIDISequence))]
        #[unsafe(method_family = none)]
        pub unsafe fn isGeneralMIDISequence(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`isGeneralMIDISequence`][Self::isGeneralMIDISequence].
        #[unsafe(method(setGeneralMIDISequence:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGeneralMIDISequence(&self, general_midi_sequence: Option<&NSNumber>);

        #[unsafe(method(musicalInstrumentCategory))]
        #[unsafe(method_family = none)]
        pub unsafe fn musicalInstrumentCategory(&self) -> Option<Retained<NSString>>;

        /// Setter for [`musicalInstrumentCategory`][Self::musicalInstrumentCategory].
        #[unsafe(method(setMusicalInstrumentCategory:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMusicalInstrumentCategory(
            &self,
            musical_instrument_category: Option<&NSString>,
        );

        #[unsafe(method(musicalInstrumentName))]
        #[unsafe(method_family = none)]
        pub unsafe fn musicalInstrumentName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`musicalInstrumentName`][Self::musicalInstrumentName].
        #[unsafe(method(setMusicalInstrumentName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMusicalInstrumentName(&self, musical_instrument_name: Option<&NSString>);
    );
}
