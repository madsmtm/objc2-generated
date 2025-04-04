//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "SKAnalysis")]
#[path = "SKAnalysis.rs"]
mod __SKAnalysis;
#[cfg(feature = "SKDocument")]
#[path = "SKDocument.rs"]
mod __SKDocument;
#[cfg(feature = "SKIndex")]
#[path = "SKIndex.rs"]
mod __SKIndex;
#[cfg(feature = "SKSearch")]
#[path = "SKSearch.rs"]
mod __SKSearch;
#[cfg(feature = "SKSummary")]
#[path = "SKSummary.rs"]
mod __SKSummary;

#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKEndTermChars;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKLanguageTypes;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKMaximumTerms;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKMinTermLength;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKProximityIndexing;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKStartTermChars;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKStopWords;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKSubstitutions;
#[cfg(feature = "SKAnalysis")]
pub use self::__SKAnalysis::kSKTermChars;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocument;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentCopyURL;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentCreate;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentCreateWithURL;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentGetName;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentGetParent;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentGetSchemeName;
#[cfg(feature = "SKDocument")]
pub use self::__SKDocument::SKDocumentGetTypeID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKDocumentID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKDocumentIndexState;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndex;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexAddDocument;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexAddDocumentWithText;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexClose;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexCompact;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexCopyDocumentForDocumentID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexCopyDocumentIDArrayForTermID;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexCopyDocumentProperties;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexCopyTermIDArrayForDocumentID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexCopyTermStringForTermID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexCreateWithMutableData;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexCreateWithURL;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexDocumentIterator;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexDocumentIteratorCopyNext;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexDocumentIteratorCreate;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexFlush;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetAnalysisProperties;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetDocumentCount;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexGetDocumentID;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexGetDocumentState;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetDocumentTermCount;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetDocumentTermFrequency;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetIndexType;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetMaximumBytesBeforeFlush;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetMaximumDocumentID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetMaximumTermID;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetTermDocumentCount;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexGetTermIDForTermString;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexMoveDocument;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexOpenWithData;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexOpenWithMutableData;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexOpenWithURL;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexRemoveDocument;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexRenameDocument;
#[cfg(all(feature = "SKDocument", feature = "SKIndex"))]
pub use self::__SKIndex::SKIndexSetDocumentProperties;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexSetMaximumBytesBeforeFlush;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKIndexType;
#[cfg(feature = "SKIndex")]
pub use self::__SKIndex::SKLoadDefaultExtractorPlugIns;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::kSKSearchOptionDefault;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::kSKSearchOptionFindSimilar;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::kSKSearchOptionNoRelevanceScores;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::kSKSearchOptionSpaceMeansOR;
#[cfg(all(feature = "SKDocument", feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKIndexCopyDocumentRefsForDocumentIDs;
#[cfg(all(feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKIndexCopyDocumentURLsForDocumentIDs;
#[cfg(all(feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKIndexCopyInfoForDocumentIDs;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearch;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchCancel;
#[cfg(all(feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKSearchCreate;
#[cfg(all(feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKSearchFindMatches;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchGroup;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchGroupCopyIndexes;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchGroupCreate;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchOptions;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchResults;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchResultsCopyMatchingTerms;
#[cfg(all(feature = "SKDocument", feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKSearchResultsCreateWithDocuments;
#[cfg(all(feature = "SKDocument", feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKSearchResultsCreateWithQuery;
#[cfg(all(feature = "SKDocument", feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKSearchResultsFilterCallBack;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchResultsGetCount;
#[cfg(all(feature = "SKDocument", feature = "SKIndex", feature = "SKSearch"))]
pub use self::__SKSearch::SKSearchResultsGetInfoInRange;
#[cfg(feature = "SKSearch")]
pub use self::__SKSearch::SKSearchType;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummary;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryCopyParagraphAtIndex;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryCopyParagraphSummaryString;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryCopySentenceAtIndex;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryCopySentenceSummaryString;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryCreateWithString;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryGetParagraphCount;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryGetParagraphSummaryInfo;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryGetSentenceCount;
#[cfg(feature = "SKSummary")]
pub use self::__SKSummary::SKSummaryGetSentenceSummaryInfo;
