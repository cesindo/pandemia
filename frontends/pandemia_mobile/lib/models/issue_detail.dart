class IssueDetail {
  String classification;
  int classificationNum;
  String creatorName;
  String desc;
  String id;
  String name;
  String primaryImage;
  List<Refs> refs;
  String registerTime;

  IssueDetail(
      {this.classification,
      this.classificationNum,
      this.creatorName,
      this.desc,
      this.id,
      this.name,
      this.primaryImage,
      this.refs,
      this.registerTime});

  IssueDetail.fromJson(Map<String, dynamic> json) {
    classification = json['classification'];
    classificationNum = json['classification_num'];
    creatorName = json['creator_name'];
    desc = json['desc'];
    id = json['id'];
    name = json['name'];
    primaryImage = json['primary_image'];
    if (json['refs'] != null) {
      refs = new List<Refs>();
      json['refs'].forEach((v) {
        refs.add(new Refs.fromJson(v));
      });
    }
    registerTime = json['register_time'];
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    data['classification'] = this.classification;
    data['classification_num'] = this.classificationNum;
    data['creator_name'] = this.creatorName;
    data['desc'] = this.desc;
    data['id'] = this.id;
    data['name'] = this.name;
    data['primary_image'] = this.primaryImage;
    if (this.refs != null) {
      data['refs'] = this.refs.map((v) => v.toJson()).toList();
    }
    data['register_time'] = this.registerTime;
    return data;
  }
}

class Refs {
  AddedBy addedBy;
  String caption;
  int classification;
  String description;
  int id;
  int ordering;
  String thumbnailUrl;
  String ts;
  String urlLink;

  Refs(
      {this.addedBy,
      this.caption,
      this.classification,
      this.description,
      this.id,
      this.ordering,
      this.thumbnailUrl,
      this.ts,
      this.urlLink});

  Refs.fromJson(Map<String, dynamic> json) {
    addedBy = json['added_by'] != null
        ? new AddedBy.fromJson(json['added_by'])
        : null;
    caption = json['caption'];
    classification = json['classification'];
    description = json['description'];
    id = json['id'];
    ordering = json['ordering'];
    thumbnailUrl = json['thumbnail_url'];
    ts = json['ts'];
    urlLink = json['url_link'];
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    if (this.addedBy != null) {
      data['added_by'] = this.addedBy.toJson();
    }
    data['caption'] = this.caption;
    data['classification'] = this.classification;
    data['description'] = this.description;
    data['id'] = this.id;
    data['ordering'] = this.ordering;
    data['thumbnail_url'] = this.thumbnailUrl;
    data['ts'] = this.ts;
    data['url_link'] = this.urlLink;
    return data;
  }
}

class AddedBy {
  String fullName;
  int id;

  AddedBy({this.fullName, this.id});

  AddedBy.fromJson(Map<String, dynamic> json) {
    fullName = json['full_name'];
    id = json['id'];
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    data['full_name'] = this.fullName;
    data['id'] = this.id;
    return data;
  }
}
